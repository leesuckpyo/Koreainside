use crate::repository::{
    validate_local_drive, RepositorySessionState, RepositorySnapshot, TreeNode,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fs::{self, OpenOptions},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;
use time::{format_description, OffsetDateTime, UtcOffset};

const MAX_EXPORT_BYTES: usize = 25 * 1024 * 1024;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Json,
}

impl ExportFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Json => "json",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportPreview {
    status: &'static str,
    format: &'static str,
    total_items: usize,
    estimated_bytes: usize,
    partial: bool,
    truncated: bool,
    warnings: Vec<String>,
    error_code: Option<&'static str>,
    message: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    status: &'static str,
    file_name: Option<String>,
    size_bytes: Option<usize>,
    total_items: usize,
    error_code: Option<&'static str>,
    message: Option<String>,
}

#[derive(Clone, Serialize)]
struct InventoryItem {
    relative_path: String,
    name: String,
    kind: &'static str,
    depth: usize,
}

#[derive(Serialize)]
struct JsonExport<'a> {
    metadata: ExportMetadata<'a>,
    data: &'a [InventoryItem],
}

#[derive(Serialize)]
struct ExportMetadata<'a> {
    service_name: &'static str,
    report_type: &'static str,
    schema_version: u8,
    app_version: &'static str,
    generated_at: &'a str,
    timezone: &'static str,
    repository_name: &'a str,
    data_completeness: &'static str,
    total_items: usize,
    excluded_count: usize,
    skipped_count: usize,
    partial: bool,
    truncated: bool,
    warnings: &'a [String],
}

#[derive(Serialize)]
struct CsvRow<'a> {
    schema_version: u8,
    generated_at_kst: &'a str,
    repository_name: String,
    data_completeness: &'static str,
    total_items: usize,
    excluded_count: usize,
    skipped_count: usize,
    partial: bool,
    truncated: bool,
    relative_path: String,
    name: String,
    kind: &'static str,
    depth: usize,
}

#[tauri::command]
pub fn preview_repository_export(
    format: ExportFormat,
    state: State<'_, RepositorySessionState>,
) -> ExportPreview {
    let snapshot = match snapshot_from_state(&state) {
        Ok(snapshot) => snapshot,
        Err(result) => return preview_error(format, result.0, result.1),
    };

    match build_export(&snapshot, format) {
        Ok((bytes, _)) => ExportPreview {
            status: "ready",
            format: format.extension(),
            total_items: snapshot.total_items,
            estimated_bytes: bytes.len(),
            partial: snapshot.partial,
            truncated: snapshot.truncated,
            warnings: snapshot.warnings,
            error_code: None,
            message: None,
        },
        Err(error) => preview_error(format, error.code, error.message),
    }
}

#[tauri::command]
pub async fn export_repository_inventory(app: AppHandle, format: ExportFormat) -> ExportResult {
    let state = app.state::<RepositorySessionState>();
    let snapshot = match snapshot_from_state(&state) {
        Ok(snapshot) => snapshot,
        Err(error) => return export_error(0, error.0, error.1),
    };

    let (bytes, file_name) = match build_export(&snapshot, format) {
        Ok(export) => export,
        Err(error) => {
            return export_error(snapshot.total_items, error.code, error.message);
        }
    };

    let extension = format.extension();
    let dialog = tauri::async_runtime::spawn_blocking(move || {
        app.dialog()
            .file()
            .add_filter(extension.to_uppercase(), &[extension])
            .set_file_name(file_name)
            .blocking_save_file()
    })
    .await;

    let selected = match dialog {
        Ok(Some(path)) => match path.into_path() {
            Ok(path) => path,
            Err(_) => {
                return export_error(
                    snapshot.total_items,
                    "INVALID_SAVE_PATH",
                    "선택한 저장 경로를 확인할 수 없습니다.",
                );
            }
        },
        Ok(None) => {
            return ExportResult {
                status: "cancelled",
                file_name: None,
                size_bytes: None,
                total_items: snapshot.total_items,
                error_code: None,
                message: None,
            };
        }
        Err(_) => {
            return export_error(
                snapshot.total_items,
                "DIALOG_FAILED",
                "파일 저장창을 열 수 없습니다.",
            );
        }
    };

    let total_items = snapshot.total_items;
    let size_bytes = bytes.len();
    match tauri::async_runtime::spawn_blocking(move || save_export(&selected, extension, &bytes))
        .await
    {
        Ok(Ok(saved_name)) => ExportResult {
            status: "saved",
            file_name: Some(saved_name),
            size_bytes: Some(size_bytes),
            total_items,
            error_code: None,
            message: None,
        },
        Ok(Err(error)) => export_error(total_items, error.code, error.message),
        Err(_) => export_error(
            total_items,
            "SAVE_FAILED",
            "파일을 저장하는 중 오류가 발생했습니다.",
        ),
    }
}

fn snapshot_from_state(
    state: &State<'_, RepositorySessionState>,
) -> Result<RepositorySnapshot, (&'static str, &'static str)> {
    let current = state.0.lock().map_err(|_| {
        (
            "SESSION_STATE_FAILED",
            "저장소 연결 상태를 읽을 수 없습니다.",
        )
    })?;
    current.clone().ok_or((
        "REPOSITORY_NOT_CONNECTED",
        "내보낼 저장소가 연결되지 않았습니다.",
    ))
}

fn build_export(
    snapshot: &RepositorySnapshot,
    format: ExportFormat,
) -> Result<(Vec<u8>, String), ExportError> {
    let mut items = flatten_tree(&snapshot.tree);
    if items.is_empty() {
        return Err(error(
            "EMPTY_REPOSITORY_INVENTORY",
            "내보낼 저장소 항목이 없습니다.",
        ));
    }
    items.sort_by(compare_items);

    let now = kst_now()?;
    let generated_at =
        format_timestamp(&now, "[year]-[month]-[day]T[hour]:[minute]:[second]+09:00")?;
    let file_stamp = format_timestamp(&now, "[year][month][day]_[hour][minute][second]_KST")?;
    let file_name = make_file_name(&file_stamp, format);

    let bytes = match format {
        ExportFormat::Csv => serialize_csv(snapshot, &items, &generated_at)?,
        ExportFormat::Json => serialize_json(snapshot, &items, &generated_at)?,
    };

    if bytes.len() > MAX_EXPORT_BYTES {
        return Err(error(
            "EXPORT_TOO_LARGE",
            "내보내기 파일이 25MB 제한을 초과합니다.",
        ));
    }

    Ok((bytes, file_name))
}

fn make_file_name(file_stamp: &str, format: ExportFormat) -> String {
    format!(
        "KoreaInside_RepositoryInventory_{file_stamp}_v1.{}",
        format.extension()
    )
}

fn serialize_csv(
    snapshot: &RepositorySnapshot,
    items: &[InventoryItem],
    generated_at: &str,
) -> Result<Vec<u8>, ExportError> {
    let mut output = vec![0xEF, 0xBB, 0xBF];
    {
        let mut writer = csv::WriterBuilder::new()
            .terminator(csv::Terminator::CRLF)
            .from_writer(&mut output);
        for item in items {
            writer
                .serialize(CsvRow {
                    schema_version: 1,
                    generated_at_kst: generated_at,
                    repository_name: excel_safe(&snapshot.repository_name),
                    data_completeness: data_completeness(snapshot),
                    total_items: snapshot.total_items,
                    excluded_count: snapshot.excluded_count,
                    skipped_count: snapshot.skipped_count,
                    partial: snapshot.partial,
                    truncated: snapshot.truncated,
                    relative_path: excel_safe(&item.relative_path),
                    name: excel_safe(&item.name),
                    kind: item.kind,
                    depth: item.depth,
                })
                .map_err(|_| error("CSV_SERIALIZATION_FAILED", "CSV를 생성할 수 없습니다."))?;
        }
        writer
            .flush()
            .map_err(|_| error("CSV_SERIALIZATION_FAILED", "CSV를 생성할 수 없습니다."))?;
    }
    Ok(output)
}

fn serialize_json(
    snapshot: &RepositorySnapshot,
    items: &[InventoryItem],
    generated_at: &str,
) -> Result<Vec<u8>, ExportError> {
    let mut output = Vec::new();
    let export = JsonExport {
        metadata: ExportMetadata {
            service_name: "Korea Inside",
            report_type: "repository_inventory",
            schema_version: 1,
            app_version: env!("CARGO_PKG_VERSION"),
            generated_at,
            timezone: "Asia/Seoul",
            repository_name: &snapshot.repository_name,
            data_completeness: data_completeness(snapshot),
            total_items: snapshot.total_items,
            excluded_count: snapshot.excluded_count,
            skipped_count: snapshot.skipped_count,
            partial: snapshot.partial,
            truncated: snapshot.truncated,
            warnings: &snapshot.warnings,
        },
        data: items,
    };
    serde_json::to_writer_pretty(&mut output, &export)
        .map_err(|_| error("JSON_SERIALIZATION_FAILED", "JSON을 생성할 수 없습니다."))?;
    output.push(b'\n');
    Ok(output)
}

fn flatten_tree(nodes: &[TreeNode]) -> Vec<InventoryItem> {
    let mut items = Vec::new();
    flatten_children(nodes, "", 1, &mut items);
    items
}

fn flatten_children(
    nodes: &[TreeNode],
    parent: &str,
    depth: usize,
    items: &mut Vec<InventoryItem>,
) {
    for node in nodes {
        let relative_path = if parent.is_empty() {
            node.name.clone()
        } else {
            format!("{parent}/{}", node.name)
        };
        items.push(InventoryItem {
            relative_path: relative_path.clone(),
            name: node.name.clone(),
            kind: node.kind,
            depth,
        });
        flatten_children(&node.children, &relative_path, depth + 1, items);
    }
}

fn compare_items(left: &InventoryItem, right: &InventoryItem) -> Ordering {
    let left_directory = left.kind == "directory";
    let right_directory = right.kind == "directory";
    right_directory
        .cmp(&left_directory)
        .then_with(|| {
            left.relative_path
                .to_lowercase()
                .cmp(&right.relative_path.to_lowercase())
        })
        .then_with(|| left.relative_path.cmp(&right.relative_path))
}

fn excel_safe(value: &str) -> String {
    let trimmed = value.trim_start();
    let dangerous =
        value.starts_with(['\t', '\r', '\n']) || trimmed.starts_with(['=', '+', '-', '@']);
    if dangerous {
        format!("'{value}")
    } else {
        value.to_string()
    }
}

fn data_completeness(snapshot: &RepositorySnapshot) -> &'static str {
    if snapshot.partial || snapshot.truncated {
        "partial"
    } else {
        "complete"
    }
}

fn kst_now() -> Result<OffsetDateTime, ExportError> {
    let offset = UtcOffset::from_hms(9, 0, 0)
        .map_err(|_| error("TIME_FAILED", "KST 시간을 생성할 수 없습니다."))?;
    Ok(OffsetDateTime::now_utc().to_offset(offset))
}

fn format_timestamp(value: &OffsetDateTime, pattern: &str) -> Result<String, ExportError> {
    let description = format_description::parse_borrowed::<2>(pattern)
        .map_err(|_| error("TIME_FAILED", "시간 형식을 생성할 수 없습니다."))?;
    value
        .format(&description)
        .map_err(|_| error("TIME_FAILED", "시간을 표시할 수 없습니다."))
}

fn save_export(path: &Path, expected_extension: &str, bytes: &[u8]) -> Result<String, ExportError> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| error("INVALID_FILE_EXTENSION", "파일 확장자가 필요합니다."))?;
    if !extension.eq_ignore_ascii_case(expected_extension) {
        return Err(error(
            "INVALID_FILE_EXTENSION",
            "선택한 파일 확장자가 출력 형식과 일치하지 않습니다.",
        ));
    }
    if path.exists() {
        return Err(error(
            "FILE_ALREADY_EXISTS",
            "같은 이름의 파일이 이미 있습니다. 다른 이름을 선택해 주십시오.",
        ));
    }
    if bytes.len() > MAX_EXPORT_BYTES {
        return Err(error(
            "EXPORT_TOO_LARGE",
            "내보내기 파일이 25MB 제한을 초과합니다.",
        ));
    }

    let parent = path
        .parent()
        .ok_or_else(|| error("INVALID_SAVE_PATH", "저장 폴더를 확인할 수 없습니다."))?;
    let parent = fs::canonicalize(parent)
        .map_err(|_| error("SAVE_ACCESS_DENIED", "저장 폴더에 접근할 수 없습니다."))?;
    validate_local_drive(&parent).map_err(|value| error(value.code, &value.message))?;

    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| error("INVALID_SAVE_PATH", "파일 이름을 확인할 수 없습니다."))?;
    let final_path = parent.join(file_name);
    if final_path.exists() {
        return Err(error(
            "FILE_ALREADY_EXISTS",
            "같은 이름의 파일이 이미 있습니다. 다른 이름을 선택해 주십시오.",
        ));
    }

    let temp_path = create_temp_path(&parent, file_name)?;
    let result = write_and_rename(&temp_path, &final_path, bytes);
    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result?;
    Ok(file_name.to_string())
}

fn create_temp_path(parent: &Path, file_name: &str) -> Result<PathBuf, ExportError> {
    for counter in 0..100u8 {
        let candidate = parent.join(format!(
            ".{file_name}.{}.{}.tmp",
            std::process::id(),
            counter
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                drop(file);
                return Ok(candidate);
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(_) => {
                return Err(error(
                    "SAVE_ACCESS_DENIED",
                    "저장 폴더에 임시파일을 만들 수 없습니다.",
                ));
            }
        }
    }
    Err(error(
        "TEMP_FILE_FAILED",
        "고유한 임시파일을 만들 수 없습니다.",
    ))
}

fn write_and_rename(temp_path: &Path, final_path: &Path, bytes: &[u8]) -> Result<(), ExportError> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(temp_path)
        .map_err(|_| error("SAVE_FAILED", "임시파일을 열 수 없습니다."))?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(bytes)
        .map_err(|_| error("SAVE_FAILED", "파일 내용을 쓸 수 없습니다."))?;
    writer
        .flush()
        .map_err(|_| error("SAVE_FAILED", "파일 쓰기를 완료할 수 없습니다."))?;
    writer
        .get_ref()
        .sync_all()
        .map_err(|_| error("SAVE_FAILED", "파일 내용을 디스크에 반영할 수 없습니다."))?;
    drop(writer);

    if final_path.exists() {
        return Err(error(
            "FILE_ALREADY_EXISTS",
            "같은 이름의 파일이 이미 있습니다. 다른 이름을 선택해 주십시오.",
        ));
    }
    fs::rename(temp_path, final_path)
        .map_err(|_| error("SAVE_FAILED", "임시파일을 최종 파일로 이동할 수 없습니다."))
}

#[derive(Debug)]
struct ExportError {
    code: &'static str,
    message: String,
}

fn error(code: &'static str, message: &str) -> ExportError {
    ExportError {
        code,
        message: message.to_string(),
    }
}

fn preview_error(
    format: ExportFormat,
    code: &'static str,
    message: impl Into<String>,
) -> ExportPreview {
    ExportPreview {
        status: "error",
        format: format.extension(),
        total_items: 0,
        estimated_bytes: 0,
        partial: false,
        truncated: false,
        warnings: Vec::new(),
        error_code: Some(code),
        message: Some(message.into()),
    }
}

fn export_error(
    total_items: usize,
    code: &'static str,
    message: impl Into<String>,
) -> ExportResult {
    ExportResult {
        status: "error",
        file_name: None,
        size_bytes: None,
        total_items,
        error_code: Some(code),
        message: Some(message.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> RepositorySnapshot {
        RepositorySnapshot {
            repository_name: "KoreaInside".to_string(),
            tree: vec![
                TreeNode {
                    name: "src".to_string(),
                    kind: "directory",
                    children: vec![TreeNode {
                        name: "=formula.js".to_string(),
                        kind: "file",
                        children: Vec::new(),
                    }],
                },
                TreeNode {
                    name: "README.md".to_string(),
                    kind: "file",
                    children: Vec::new(),
                },
            ],
            total_items: 3,
            excluded_count: 1,
            skipped_count: 0,
            partial: false,
            truncated: false,
            warnings: Vec::new(),
        }
    }

    #[test]
    fn flattens_tree_with_forward_slash_relative_paths() {
        let items = flatten_tree(&snapshot().tree);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].relative_path, "src");
        assert_eq!(items[0].depth, 1);
        assert_eq!(items[1].relative_path, "src/=formula.js");
        assert_eq!(items[1].depth, 2);
    }

    #[test]
    fn csv_has_bom_crlf_and_sanitizes_formula_cells() {
        let snapshot = snapshot();
        let items = flatten_tree(&snapshot.tree);
        let csv = serialize_csv(&snapshot, &items, "2026-07-11T12:00:00+09:00").unwrap();
        assert!(csv.starts_with(&[0xEF, 0xBB, 0xBF]));
        assert!(csv.windows(2).any(|window| window == b"\r\n"));
        assert!(!csv.windows(2).any(|window| window == b"\n\n"));
        let text = String::from_utf8(csv).unwrap();
        assert!(text.contains("'=formula.js"));
        assert_eq!(excel_safe("  +SUM(A1:A2)"), "'  +SUM(A1:A2)");
        assert_eq!(excel_safe("safe"), "safe");
    }

    #[test]
    fn json_is_pretty_reparseable_and_has_no_absolute_path_field() {
        let snapshot = snapshot();
        let items = flatten_tree(&snapshot.tree);
        let json = serialize_json(&snapshot, &items, "2026-07-11T12:00:00+09:00").unwrap();
        let value: serde_json::Value = serde_json::from_slice(&json).unwrap();
        assert_eq!(value["metadata"]["schema_version"], 1);
        assert_eq!(value["data"][1]["relative_path"], "src/=formula.js");
        let text = String::from_utf8(json).unwrap();
        assert!(text.contains("\n  \"metadata\""));
        assert!(!text.contains("root_path"));
        assert!(!text.contains("C:\\\\"));
    }

    #[test]
    fn creates_approved_file_names() {
        assert_eq!(
            make_file_name("20260711_120000_KST", ExportFormat::Csv),
            "KoreaInside_RepositoryInventory_20260711_120000_KST_v1.csv"
        );
        assert_eq!(
            make_file_name("20260711_120000_KST", ExportFormat::Json),
            "KoreaInside_RepositoryInventory_20260711_120000_KST_v1.json"
        );
    }

    #[test]
    fn rejects_empty_inventory() {
        let mut snapshot = snapshot();
        snapshot.tree.clear();
        snapshot.total_items = 0;
        let result = build_export(&snapshot, ExportFormat::Json).unwrap_err();
        assert_eq!(result.code, "EMPTY_REPOSITORY_INVENTORY");
    }

    #[test]
    fn saves_once_and_rejects_collision_and_wrong_extension() {
        let directory = std::env::temp_dir().join(format!(
            "korea-inside-admin-export-test-{}-{}",
            std::process::id(),
            OffsetDateTime::now_utc().unix_timestamp_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        let json_path = directory.join("inventory.json");
        let wrong_path = directory.join("inventory.txt");

        assert_eq!(
            save_export(&json_path, "json", b"{}\n").unwrap(),
            "inventory.json"
        );
        assert_eq!(
            save_export(&json_path, "json", b"{}\n").unwrap_err().code,
            "FILE_ALREADY_EXISTS"
        );
        assert_eq!(
            save_export(&wrong_path, "json", b"{}\n").unwrap_err().code,
            "INVALID_FILE_EXTENSION"
        );

        fs::remove_dir_all(directory).unwrap();
    }
}
