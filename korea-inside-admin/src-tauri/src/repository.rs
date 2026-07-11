use serde::Serialize;
use serde_json::Value;
use std::{
    cmp::Ordering,
    fs::{self, DirEntry, File},
    io::{self, Read},
    os::windows::fs::MetadataExt,
    path::{Component, Path, PathBuf, Prefix},
    sync::Mutex,
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;
use windows_sys::Win32::{
    Storage::FileSystem::{GetDriveTypeW, FILE_ATTRIBUTE_REPARSE_POINT},
    System::WindowsProgramming::{DRIVE_FIXED, DRIVE_REMOTE, DRIVE_REMOVABLE},
};

const MAX_DEPTH: usize = 12;
const MAX_ITEMS: usize = 5_000;
const MAX_JSON_BYTES: u64 = 64 * 1024;
const MAX_GIT_FILE_BYTES: u64 = 4 * 1024;
const MAX_WARNINGS: usize = 25;

const EXCLUDED_DIRECTORIES: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    ".vercel",
    "dist",
    "dist-ssr",
    "coverage",
    ".cache",
    ".parcel-cache",
    ".vite",
    "tmp",
    "temp",
];

const EXCLUDED_FILES: &[&str] = &["desktop.ini", "Thumbs.db", ".DS_Store"];

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectRepositoryResult {
    status: &'static str,
    root_path: Option<String>,
    repository_name: Option<String>,
    tree: Vec<TreeNode>,
    total_items: usize,
    excluded_count: usize,
    skipped_count: usize,
    partial: bool,
    truncated: bool,
    warnings: Vec<String>,
    error_code: Option<&'static str>,
    message: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct TreeNode {
    pub(crate) name: String,
    pub(crate) kind: &'static str,
    pub(crate) children: Vec<TreeNode>,
}

#[derive(Clone)]
pub(crate) struct RepositorySnapshot {
    pub(crate) repository_name: String,
    pub(crate) tree: Vec<TreeNode>,
    pub(crate) total_items: usize,
    pub(crate) excluded_count: usize,
    pub(crate) skipped_count: usize,
    pub(crate) partial: bool,
    pub(crate) truncated: bool,
    pub(crate) warnings: Vec<String>,
}

#[derive(Default)]
pub(crate) struct RepositorySessionState(pub(crate) Mutex<Option<RepositorySnapshot>>);

pub(crate) struct UserError {
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

struct ScanContext {
    root: PathBuf,
    total_items: usize,
    excluded_count: usize,
    skipped_count: usize,
    partial: bool,
    truncated: bool,
    warnings: Vec<String>,
}

struct Candidate {
    entry: DirEntry,
    name: String,
    is_directory: bool,
}

impl SelectRepositoryResult {
    fn cancelled() -> Self {
        Self {
            status: "cancelled",
            root_path: None,
            repository_name: None,
            tree: Vec::new(),
            total_items: 0,
            excluded_count: 0,
            skipped_count: 0,
            partial: false,
            truncated: false,
            warnings: Vec::new(),
            error_code: None,
            message: None,
        }
    }

    fn error(error: UserError) -> Self {
        Self {
            status: "error",
            root_path: None,
            repository_name: None,
            tree: Vec::new(),
            total_items: 0,
            excluded_count: 0,
            skipped_count: 0,
            partial: false,
            truncated: false,
            warnings: Vec::new(),
            error_code: Some(error.code),
            message: Some(error.message),
        }
    }
}

#[tauri::command]
pub async fn select_repository(app: AppHandle) -> SelectRepositoryResult {
    let dialog_app = app.clone();
    let dialog_result = tauri::async_runtime::spawn_blocking(move || {
        dialog_app.dialog().file().blocking_pick_folder()
    })
    .await;

    let selected = match dialog_result {
        Ok(Some(path)) => match path.into_path() {
            Ok(path) => path,
            Err(_) => {
                return SelectRepositoryResult::error(user_error(
                    "INVALID_SELECTION",
                    "선택한 폴더 경로를 확인할 수 없습니다.",
                ));
            }
        },
        Ok(None) => return SelectRepositoryResult::cancelled(),
        Err(_) => {
            return SelectRepositoryResult::error(user_error(
                "DIALOG_FAILED",
                "폴더 선택창을 열 수 없습니다.",
            ));
        }
    };

    match tauri::async_runtime::spawn_blocking(move || inspect_repository(selected)).await {
        Ok(Ok((result, snapshot))) => {
            let state = app.state::<RepositorySessionState>();
            if let Ok(mut current) = state.0.lock() {
                *current = Some(snapshot);
            } else {
                return SelectRepositoryResult::error(user_error(
                    "SESSION_STATE_FAILED",
                    "저장소 연결 상태를 저장할 수 없습니다.",
                ));
            }
            result
        }
        Ok(Err(error)) => SelectRepositoryResult::error(error),
        Err(_) => SelectRepositoryResult::error(user_error(
            "SCAN_FAILED",
            "저장소 구조를 확인하는 중 오류가 발생했습니다.",
        )),
    }
}

#[tauri::command]
pub fn disconnect_repository(state: State<'_, RepositorySessionState>) -> Result<(), String> {
    let mut current = state
        .0
        .lock()
        .map_err(|_| "저장소 연결 상태를 삭제할 수 없습니다.".to_string())?;
    *current = None;
    Ok(())
}

fn inspect_repository(
    selected: PathBuf,
) -> Result<(SelectRepositoryResult, RepositorySnapshot), UserError> {
    let selected_metadata = fs::symlink_metadata(&selected)
        .map_err(|_| user_error("ROOT_ACCESS_DENIED", "선택한 폴더에 접근할 수 없습니다."))?;

    if !selected_metadata.is_dir() {
        return Err(user_error("INVALID_SELECTION", "폴더를 선택해 주십시오."));
    }

    if is_reparse_point(&selected_metadata) {
        return Err(user_error(
            "REPARSE_ROOT_NOT_ALLOWED",
            "심볼릭 링크 또는 연결 지점은 저장소 루트로 선택할 수 없습니다.",
        ));
    }

    let root = fs::canonicalize(&selected).map_err(|_| {
        user_error(
            "ROOT_ACCESS_DENIED",
            "선택한 폴더의 실제 경로를 확인할 수 없습니다.",
        )
    })?;

    validate_local_drive(&root)?;
    validate_repository(&root)?;

    let repository_name = root
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "Korea Inside".to_string());

    let mut context = ScanContext {
        root: root.clone(),
        total_items: 0,
        excluded_count: 0,
        skipped_count: 0,
        partial: false,
        truncated: false,
        warnings: Vec::new(),
    };

    let tree = scan_directory(&root, 1, &mut context).map_err(|_| {
        user_error(
            "ROOT_ACCESS_DENIED",
            "저장소 루트의 파일과 폴더를 읽을 수 없습니다.",
        )
    })?;

    let snapshot = RepositorySnapshot {
        repository_name: repository_name.clone(),
        tree: tree.clone(),
        total_items: context.total_items,
        excluded_count: context.excluded_count,
        skipped_count: context.skipped_count,
        partial: context.partial,
        truncated: context.truncated,
        warnings: context.warnings.clone(),
    };

    let result = SelectRepositoryResult {
        status: "connected",
        root_path: Some(display_path(&root)),
        repository_name: Some(repository_name),
        tree,
        total_items: context.total_items,
        excluded_count: context.excluded_count,
        skipped_count: context.skipped_count,
        partial: context.partial,
        truncated: context.truncated,
        warnings: context.warnings,
        error_code: None,
        message: None,
    };

    Ok((result, snapshot))
}

pub(crate) fn validate_local_drive(path: &Path) -> Result<(), UserError> {
    let prefix = path
        .components()
        .next()
        .and_then(|component| match component {
            Component::Prefix(prefix) => Some(prefix.kind()),
            _ => None,
        });

    let drive_letter = match prefix {
        Some(Prefix::Disk(letter)) | Some(Prefix::VerbatimDisk(letter)) => letter,
        Some(Prefix::UNC(_, _)) | Some(Prefix::VerbatimUNC(_, _)) => {
            return Err(user_error(
                "NETWORK_PATH_NOT_ALLOWED",
                "네트워크 저장소는 연결할 수 없습니다.",
            ));
        }
        _ => {
            return Err(user_error(
                "LOCAL_DRIVE_REQUIRED",
                "로컬 Windows 드라이브의 폴더를 선택해 주십시오.",
            ));
        }
    };

    let root = format!("{}:\\", char::from(drive_letter));
    let mut wide: Vec<u16> = root.encode_utf16().collect();
    wide.push(0);

    // SAFETY: `wide` is a null-terminated UTF-16 drive root and remains alive for the call.
    let drive_type = unsafe { GetDriveTypeW(wide.as_ptr()) };

    match drive_type {
        DRIVE_FIXED | DRIVE_REMOVABLE => Ok(()),
        DRIVE_REMOTE => Err(user_error(
            "NETWORK_PATH_NOT_ALLOWED",
            "네트워크 저장소는 연결할 수 없습니다.",
        )),
        _ => Err(user_error(
            "LOCAL_DRIVE_REQUIRED",
            "지원되는 로컬 드라이브의 폴더를 선택해 주십시오.",
        )),
    }
}

fn validate_repository(root: &Path) -> Result<(), UserError> {
    require_readable_regular_file(&root.join("PROJECT.md"))?;
    require_readable_regular_file(&root.join("AGENTS.md"))?;

    let git_path = root.join(".git");
    let git_metadata = fs::symlink_metadata(&git_path).map_err(|_| invalid_repository())?;
    if is_reparse_point(&git_metadata) {
        return Err(invalid_repository());
    }
    if git_metadata.is_file() {
        let contents =
            read_limited_text(&git_path, MAX_GIT_FILE_BYTES).map_err(|_| invalid_repository())?;
        let gitdir = contents
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("gitdir:"))
            .map(str::trim)
            .filter(|value| !value.is_empty());
        if gitdir.is_none() {
            return Err(invalid_repository());
        }
    } else if !git_metadata.is_dir() {
        return Err(invalid_repository());
    }

    require_regular_directory(&root.join("korea-inside-admin"))?;

    let signatures = [
        json_property_equals(&root.join("package.json"), "name", "korea-inside"),
        json_property_equals(
            &root.join("korea-inside-admin/package.json"),
            "name",
            "korea-inside-admin",
        ),
        json_property_equals(
            &root.join("korea-inside-admin/src-tauri/tauri.conf.json"),
            "identifier",
            "com.getkoreainside.admin",
        ),
        is_regular_file(&root.join("docs/product-constitution.md")),
        ["index.html", "style.css", "common.js"]
            .iter()
            .all(|name| is_regular_file(&root.join(name))),
    ];

    if signatures.into_iter().filter(|matched| *matched).count() < 3 {
        return Err(invalid_repository());
    }

    Ok(())
}

fn require_readable_regular_file(path: &Path) -> Result<(), UserError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| invalid_repository())?;
    if !metadata.is_file() || is_reparse_point(&metadata) {
        return Err(invalid_repository());
    }
    File::open(path).map_err(|_| invalid_repository())?;
    Ok(())
}

fn require_regular_directory(path: &Path) -> Result<(), UserError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| invalid_repository())?;
    if metadata.is_dir() && !is_reparse_point(&metadata) {
        Ok(())
    } else {
        Err(invalid_repository())
    }
}

fn json_property_equals(path: &Path, property: &str, expected: &str) -> bool {
    let contents = match read_limited_text(path, MAX_JSON_BYTES) {
        Ok(contents) => contents,
        Err(_) => return false,
    };

    serde_json::from_str::<Value>(&contents)
        .ok()
        .and_then(|value| {
            value
                .get(property)
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .is_some_and(|value| value == expected)
}

fn read_limited_text(path: &Path, max_bytes: u64) -> io::Result<String> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || is_reparse_point(&metadata) || metadata.len() > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "validation file is invalid or too large",
        ));
    }

    let file = File::open(path)?;
    let mut contents = String::new();
    file.take(max_bytes + 1).read_to_string(&mut contents)?;
    if contents.len() as u64 > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "validation file is too large",
        ));
    }
    Ok(contents)
}

fn scan_directory(
    path: &Path,
    depth: usize,
    context: &mut ScanContext,
) -> io::Result<Vec<TreeNode>> {
    let mut candidates = Vec::new();

    for entry_result in fs::read_dir(path)? {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(_) => {
                record_skip(context, path, "항목 정보를 읽을 수 없어 건너뛰었습니다.");
                continue;
            }
        };

        let name = entry.file_name().to_string_lossy().into_owned();
        let entry_path = entry.path();
        let metadata = match fs::symlink_metadata(&entry_path) {
            Ok(metadata) => metadata,
            Err(_) => {
                record_skip(
                    context,
                    &entry_path,
                    "항목 속성을 읽을 수 없어 건너뛰었습니다.",
                );
                continue;
            }
        };

        if is_reparse_point(&metadata) {
            record_skip(
                context,
                &entry_path,
                "연결 지점은 안전을 위해 건너뛰었습니다.",
            );
            continue;
        }

        let is_directory = metadata.is_dir();
        if (is_directory && is_excluded_directory(&name))
            || (metadata.is_file() && is_excluded_file(&name))
        {
            context.excluded_count += 1;
            continue;
        }

        if !is_directory && !metadata.is_file() {
            record_skip(
                context,
                &entry_path,
                "지원하지 않는 항목 형식이라 건너뛰었습니다.",
            );
            continue;
        }

        candidates.push(Candidate {
            entry,
            name,
            is_directory,
        });
    }

    candidates.sort_by(compare_candidates);

    let mut nodes = Vec::new();
    for candidate in candidates {
        if context.total_items >= MAX_ITEMS {
            context.truncated = true;
            break;
        }

        context.total_items += 1;
        let mut children = Vec::new();

        if candidate.is_directory {
            if depth < MAX_DEPTH {
                match scan_directory(&candidate.entry.path(), depth + 1, context) {
                    Ok(scanned) => children = scanned,
                    Err(_) => record_skip(
                        context,
                        &candidate.entry.path(),
                        "폴더를 읽을 수 없어 하위 항목을 건너뛰었습니다.",
                    ),
                }
            } else {
                context.truncated = true;
            }
        }

        nodes.push(TreeNode {
            name: candidate.name,
            kind: if candidate.is_directory {
                "directory"
            } else {
                "file"
            },
            children,
        });

        if context.truncated && context.total_items >= MAX_ITEMS {
            break;
        }
    }

    Ok(nodes)
}

fn compare_candidates(left: &Candidate, right: &Candidate) -> Ordering {
    right
        .is_directory
        .cmp(&left.is_directory)
        .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
        .then_with(|| left.name.cmp(&right.name))
}

fn record_skip(context: &mut ScanContext, path: &Path, reason: &str) {
    context.skipped_count += 1;
    context.partial = true;
    if context.warnings.len() < MAX_WARNINGS {
        let relative = path
            .strip_prefix(&context.root)
            .map(display_path)
            .unwrap_or_else(|_| "알 수 없는 항목".to_string());
        context.warnings.push(format!("{relative}: {reason}"));
    }
}

fn is_regular_file(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && !is_reparse_point(&metadata))
}

fn is_reparse_point(metadata: &fs::Metadata) -> bool {
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

fn is_excluded_directory(name: &str) -> bool {
    EXCLUDED_DIRECTORIES
        .iter()
        .any(|excluded| name.eq_ignore_ascii_case(excluded))
}

fn is_excluded_file(name: &str) -> bool {
    EXCLUDED_FILES
        .iter()
        .any(|excluded| name.eq_ignore_ascii_case(excluded))
}

fn display_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    value
        .strip_prefix(r"\\?\")
        .unwrap_or(value.as_ref())
        .to_string()
}

fn invalid_repository() -> UserError {
    user_error(
        "INVALID_REPOSITORY",
        "선택한 폴더는 유효한 Korea Inside 저장소 루트가 아닙니다.",
    )
}

fn user_error(code: &'static str, message: &str) -> UserError {
    UserError {
        code,
        message: message.to_string(),
    }
}
