use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, Read},
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
};
use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

const MAX_HTML_BYTES: u64 = 768 * 1024;
const MAX_TEXT_BYTES: u64 = 512 * 1024;
const MAX_SCAN_DEPTH: usize = 12;
const MAX_HTML_FILES: usize = 1_000;
const MAX_GLOBAL_LIST_ITEMS: usize = 200;
const BASE_URL: &str = "https://www.getkoreainside.com/";
const EXCLUDED_SCAN_DIRECTORIES: &[&str] = &[
    ".git",
    ".vercel",
    "node_modules",
    "target",
    "dist",
    "dist-ssr",
    "coverage",
    ".cache",
    ".parcel-cache",
    ".vite",
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteStatusReport {
    status: &'static str,
    summary: SiteStatusSummary,
    pages: Vec<HtmlPageStatus>,
    global: GlobalSiteChecks,
    error_code: Option<&'static str>,
    message: Option<String>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteStatusSummary {
    checked_html_count: usize,
    ok_count: usize,
    warning_count: usize,
    error_count: usize,
    sitemap_exists: bool,
    robots_txt_exists: bool,
    vercel_json_exists: bool,
    vercelignore_exists: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlPageStatus {
    relative_path: String,
    title_exists: bool,
    title: Option<String>,
    meta_description_exists: bool,
    meta_description: Option<String>,
    canonical_exists: bool,
    canonical: Option<String>,
    h1_count: usize,
    robots_noindex: bool,
    sitemap_included: bool,
    page_classification: PageClassification,
    issues: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PageClassification {
    Public,
    Excluded,
    ReviewRequired,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSiteChecks {
    sitemap_xml_exists: bool,
    robots_txt_exists: bool,
    sitemap_url_count: usize,
    sitemap_has_duplicate_urls: bool,
    duplicate_sitemap_urls: Vec<String>,
    robots_has_sitemap: bool,
    vercel_json_exists: bool,
    vercelignore_exists: bool,
    public_html_not_in_sitemap: Vec<String>,
    sitemap_urls_without_local_file: Vec<String>,
    deployment_exclusion_evidence: Vec<DeploymentExclusionEvidence>,
    warnings: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentExclusionEvidence {
    relative_path: String,
    page_classification: PageClassification,
    evidence: Vec<String>,
}

#[derive(Default)]
struct SiteInputs {
    sitemap_urls: Vec<String>,
    sitemap_relatives: HashSet<String>,
    duplicate_sitemap_urls: Vec<String>,
    robots_has_sitemap: bool,
    robots_disallow_rules: Vec<String>,
    vercelignore_patterns: Vec<IgnorePattern>,
    vercelignore_exists: bool,
    sitemap_exists: bool,
    robots_exists: bool,
    vercel_json_exists: bool,
    global_warnings: Vec<String>,
}

#[derive(Clone)]
struct IgnorePattern {
    original: String,
    normalized: String,
    directory_only: bool,
    supported: bool,
}

#[derive(Default)]
struct HtmlMetadata {
    title: Option<String>,
    meta_description: Option<String>,
    canonical: Option<String>,
    h1_count: usize,
    robots_noindex: bool,
}

#[tauri::command]
pub fn get_site_status_report(repository_root: String) -> SiteStatusReport {
    let root = match validate_repository_root(&repository_root) {
        Ok(root) => root,
        Err(error) => return SiteStatusReport::error(error.code, error.message),
    };

    match build_site_status_report(&root) {
        Ok(report) => report,
        Err(error) => SiteStatusReport::error(error.code, error.message),
    }
}

impl SiteStatusReport {
    fn error(code: &'static str, message: String) -> Self {
        Self {
            status: "error",
            summary: SiteStatusSummary::default(),
            pages: Vec::new(),
            global: GlobalSiteChecks::default(),
            error_code: Some(code),
            message: Some(message),
        }
    }
}

fn build_site_status_report(root: &Path) -> Result<SiteStatusReport, SiteStatusError> {
    let inputs = collect_site_inputs(root);
    let mut html_paths = Vec::new();
    let mut scan_warnings = Vec::new();
    collect_html_paths(root, root, 0, &mut html_paths, &mut scan_warnings);
    html_paths.sort();

    let mut pages = Vec::new();
    for path in html_paths.into_iter().take(MAX_HTML_FILES) {
        pages.push(analyze_html_page(root, &path, &inputs));
    }
    if pages.len() == MAX_HTML_FILES {
        scan_warnings.push("html_scan_truncated".to_string());
    }

    let html_relative_paths: HashSet<String> = pages
        .iter()
        .map(|page| page.relative_path.clone())
        .collect();

    let global = build_global_checks(root, &pages, &html_relative_paths, inputs, scan_warnings);
    let summary = build_summary(&pages, &global);

    Ok(SiteStatusReport {
        status: "ok",
        summary,
        pages,
        global,
        error_code: None,
        message: None,
    })
}

fn collect_site_inputs(root: &Path) -> SiteInputs {
    let sitemap_path = root.join("sitemap.xml");
    let robots_path = root.join("robots.txt");
    let vercel_json_path = root.join("vercel.json");
    let vercelignore_path = root.join(".vercelignore");

    let mut inputs = SiteInputs {
        sitemap_exists: is_readable_regular_file(&sitemap_path),
        robots_exists: is_readable_regular_file(&robots_path),
        vercel_json_exists: is_readable_regular_file(&vercel_json_path),
        vercelignore_exists: is_readable_regular_file(&vercelignore_path),
        ..Default::default()
    };

    if inputs.sitemap_exists {
        match read_limited_utf8(&sitemap_path, MAX_TEXT_BYTES) {
            Ok(contents) => {
                inputs.sitemap_urls = extract_xml_elements(&contents, "loc");
                let mut seen = HashSet::new();
                let mut duplicates = HashSet::new();
                for url in &inputs.sitemap_urls {
                    if !seen.insert(url.to_string()) {
                        duplicates.insert(url.to_string());
                    }
                    if let Some(relative) = sitemap_url_to_relative_path(url) {
                        match checked_child_path(root, &relative) {
                            Ok(_) => {
                                inputs.sitemap_relatives.insert(relative);
                            }
                            Err(_) => inputs
                                .global_warnings
                                .push("sitemap_url_outside_repository".to_string()),
                        }
                    }
                }
                inputs.duplicate_sitemap_urls = sorted_limited(duplicates);
            }
            Err(_) => inputs
                .global_warnings
                .push("sitemap_xml_read_failed".to_string()),
        }
    }

    if inputs.robots_exists {
        match read_limited_utf8(&robots_path, MAX_TEXT_BYTES) {
            Ok(contents) => {
                inputs.robots_has_sitemap = robots_has_sitemap(&contents);
                inputs.robots_disallow_rules = parse_robots_disallow_rules(&contents);
            }
            Err(_) => inputs
                .global_warnings
                .push("robots_txt_read_failed".to_string()),
        }
    }

    if inputs.vercelignore_exists {
        match read_limited_utf8(&vercelignore_path, MAX_TEXT_BYTES) {
            Ok(contents) => inputs.vercelignore_patterns = parse_vercelignore(&contents),
            Err(_) => inputs
                .global_warnings
                .push("vercelignore_read_failed".to_string()),
        }
    }

    inputs
}

fn analyze_html_page(root: &Path, path: &Path, inputs: &SiteInputs) -> HtmlPageStatus {
    let relative_path = relative_path(root, path);
    let sitemap_included = inputs.sitemap_relatives.contains(&relative_path);
    let vercelignore_evidence =
        vercelignore_evidence(&relative_path, &inputs.vercelignore_patterns);
    let unsupported_vercelignore_evidence =
        unsupported_vercelignore_evidence(&relative_path, &inputs.vercelignore_patterns);
    let robots_disallowed = robots_disallows(&relative_path, &inputs.robots_disallow_rules);

    let mut issues = Vec::new();
    let metadata = match read_limited_utf8(path, MAX_HTML_BYTES) {
        Ok(contents) => analyze_html(&contents),
        Err(error) => {
            issues.push(if error.kind() == io::ErrorKind::InvalidData {
                "html_invalid_or_too_large".to_string()
            } else {
                "html_read_failed".to_string()
            });
            HtmlMetadata::default()
        }
    };

    if is_blank_option(metadata.title.as_deref()) {
        issues.push("missing_title".to_string());
    }
    if is_blank_option(metadata.meta_description.as_deref()) {
        issues.push("missing_meta_description".to_string());
    }
    if is_blank_option(metadata.canonical.as_deref()) {
        issues.push("missing_canonical".to_string());
    }
    match metadata.h1_count {
        0 => issues.push("missing_h1".to_string()),
        1 => {}
        _ => issues.push("multiple_h1".to_string()),
    }
    if metadata.robots_noindex {
        issues.push("robots_noindex".to_string());
    }
    if robots_disallowed {
        issues.push("robots_txt_disallow".to_string());
    }
    if !unsupported_vercelignore_evidence.is_empty() {
        issues.push("unsupported_vercelignore_pattern_requires_review".to_string());
    } else if !vercelignore_evidence.is_empty() {
        issues.push("excluded_by_vercelignore".to_string());
    }
    if !sitemap_included && vercelignore_evidence.is_empty() && !metadata.robots_noindex {
        issues.push("not_in_sitemap_review_required".to_string());
    }
    if sitemap_included && !vercelignore_evidence.is_empty() {
        issues.push("sitemap_includes_deploy_excluded_page".to_string());
    }

    let page_classification = classify_page(
        sitemap_included,
        !vercelignore_evidence.is_empty(),
        !unsupported_vercelignore_evidence.is_empty(),
        metadata.robots_noindex,
        robots_disallowed,
    );

    HtmlPageStatus {
        relative_path,
        title_exists: metadata
            .title
            .as_deref()
            .is_some_and(|value| !value.is_empty()),
        title: metadata.title,
        meta_description_exists: metadata
            .meta_description
            .as_deref()
            .is_some_and(|value| !value.is_empty()),
        meta_description: metadata.meta_description,
        canonical_exists: metadata
            .canonical
            .as_deref()
            .is_some_and(|value| !value.is_empty()),
        canonical: metadata.canonical,
        h1_count: metadata.h1_count,
        robots_noindex: metadata.robots_noindex,
        sitemap_included,
        page_classification,
        issues,
    }
}

fn build_global_checks(
    root: &Path,
    pages: &[HtmlPageStatus],
    html_relative_paths: &HashSet<String>,
    inputs: SiteInputs,
    scan_warnings: Vec<String>,
) -> GlobalSiteChecks {
    let mut public_html_not_in_sitemap = Vec::new();
    let mut sitemap_urls_without_local_file = Vec::new();
    let mut deployment_exclusion_evidence = Vec::new();

    for page in pages {
        if page.page_classification == PageClassification::ReviewRequired
            && !page.sitemap_included
            && !page.robots_noindex
        {
            push_limited(&mut public_html_not_in_sitemap, page.relative_path.clone());
        }

        let evidence = page_deployment_evidence(root, page, &inputs);
        if !evidence.is_empty() {
            push_limited(
                &mut deployment_exclusion_evidence,
                DeploymentExclusionEvidence {
                    relative_path: page.relative_path.clone(),
                    page_classification: page.page_classification,
                    evidence,
                },
            );
        }
    }

    for relative in &inputs.sitemap_relatives {
        if !html_relative_paths.contains(relative) {
            push_limited(&mut sitemap_urls_without_local_file, relative.clone());
        }
    }

    public_html_not_in_sitemap.sort();
    sitemap_urls_without_local_file.sort();
    deployment_exclusion_evidence.sort_by(|left, right| {
        left.relative_path
            .to_lowercase()
            .cmp(&right.relative_path.to_lowercase())
            .then_with(|| left.relative_path.cmp(&right.relative_path))
    });

    let mut warnings = inputs.global_warnings;
    warnings.extend(scan_warnings);
    warnings.sort();
    warnings.dedup();

    GlobalSiteChecks {
        sitemap_xml_exists: inputs.sitemap_exists,
        robots_txt_exists: inputs.robots_exists,
        sitemap_url_count: inputs.sitemap_urls.len(),
        sitemap_has_duplicate_urls: !inputs.duplicate_sitemap_urls.is_empty(),
        duplicate_sitemap_urls: inputs.duplicate_sitemap_urls,
        robots_has_sitemap: inputs.robots_has_sitemap,
        vercel_json_exists: inputs.vercel_json_exists,
        vercelignore_exists: inputs.vercelignore_exists,
        public_html_not_in_sitemap,
        sitemap_urls_without_local_file,
        deployment_exclusion_evidence,
        warnings,
    }
}

fn build_summary(pages: &[HtmlPageStatus], global: &GlobalSiteChecks) -> SiteStatusSummary {
    let mut summary = SiteStatusSummary {
        checked_html_count: pages.len(),
        sitemap_exists: global.sitemap_xml_exists,
        robots_txt_exists: global.robots_txt_exists,
        vercel_json_exists: global.vercel_json_exists,
        vercelignore_exists: global.vercelignore_exists,
        ..Default::default()
    };

    for page in pages {
        if page
            .issues
            .iter()
            .any(|issue| issue == "html_read_failed" || issue == "html_invalid_or_too_large")
        {
            summary.error_count += 1;
        } else if page.issues.is_empty() {
            summary.ok_count += 1;
        } else {
            summary.warning_count += 1;
        }
    }

    summary
}

fn page_deployment_evidence(
    root: &Path,
    page: &HtmlPageStatus,
    inputs: &SiteInputs,
) -> Vec<String> {
    if page.page_classification == PageClassification::Public {
        return Vec::new();
    }

    let mut evidence = Vec::new();

    evidence.extend(vercelignore_evidence(
        &page.relative_path,
        &inputs.vercelignore_patterns,
    ));
    evidence.extend(unsupported_vercelignore_evidence(
        &page.relative_path,
        &inputs.vercelignore_patterns,
    ));
    if page.sitemap_included {
        evidence.push("sitemap.xml includes this page".to_string());
    } else {
        evidence.push("sitemap.xml does not include this page".to_string());
    }
    if page.robots_noindex {
        evidence.push("meta robots contains noindex".to_string());
    }
    if robots_disallows(&page.relative_path, &inputs.robots_disallow_rules) {
        evidence.push("robots.txt has a matching disallow rule".to_string());
    }
    evidence.extend(document_evidence(root, &page.relative_path));

    evidence
}

fn classify_page(
    sitemap_included: bool,
    excluded_by_vercelignore: bool,
    unsupported_vercelignore_pattern: bool,
    robots_noindex: bool,
    robots_disallowed: bool,
) -> PageClassification {
    if unsupported_vercelignore_pattern {
        PageClassification::ReviewRequired
    } else if excluded_by_vercelignore {
        PageClassification::Excluded
    } else if sitemap_included && !robots_noindex && !robots_disallowed {
        PageClassification::Public
    } else {
        PageClassification::ReviewRequired
    }
}

fn collect_html_paths(
    root: &Path,
    directory: &Path,
    depth: usize,
    html_paths: &mut Vec<PathBuf>,
    warnings: &mut Vec<String>,
) {
    if depth > MAX_SCAN_DEPTH || html_paths.len() >= MAX_HTML_FILES {
        warnings.push("html_scan_depth_or_count_limit_reached".to_string());
        return;
    }

    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => {
            warnings.push(format!(
                "directory_read_failed:{}",
                relative_path(root, directory)
            ));
            return;
        }
    };

    for entry in entries.flatten() {
        if html_paths.len() >= MAX_HTML_FILES {
            warnings.push("html_scan_count_limit_reached".to_string());
            return;
        }

        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => {
                warnings.push(format!(
                    "metadata_read_failed:{}",
                    relative_path(root, &path)
                ));
                continue;
            }
        };

        if is_reparse_point(&metadata) {
            warnings.push(format!(
                "reparse_point_skipped:{}",
                relative_path(root, &path)
            ));
            continue;
        }

        if metadata.is_dir() {
            if should_skip_scan_directory(&name) {
                continue;
            }
            collect_html_paths(root, &path, depth + 1, html_paths, warnings);
        } else if metadata.is_file() && is_html_file(&path) && canonical_inside(root, &path) {
            html_paths.push(path);
        }
    }
}

fn analyze_html(contents: &str) -> HtmlMetadata {
    let mut metadata = HtmlMetadata {
        title: extract_title(contents),
        ..Default::default()
    };

    for tag in html_tags(contents) {
        let Some((name, attrs)) = parse_tag(tag) else {
            continue;
        };

        match name.as_str() {
            "h1" => metadata.h1_count += 1,
            "meta" => {
                let attr_name = attrs.get("name").map(|value| value.to_lowercase());
                if attr_name.as_deref() == Some("description") {
                    metadata.meta_description =
                        attrs.get("content").map(|value| normalize_text(value));
                } else if attr_name.as_deref() == Some("robots")
                    && attrs
                        .get("content")
                        .is_some_and(|value| value.to_lowercase().contains("noindex"))
                {
                    metadata.robots_noindex = true;
                }
            }
            "link" => {
                let rel = attrs.get("rel").map(|value| value.to_lowercase());
                if rel
                    .as_deref()
                    .is_some_and(|value| value.split_whitespace().any(|item| item == "canonical"))
                {
                    metadata.canonical = attrs.get("href").map(|value| normalize_text(value));
                }
            }
            _ => {}
        }
    }

    metadata
}

fn html_tags(contents: &str) -> Vec<&str> {
    let mut tags = Vec::new();
    let mut cursor = 0;
    while let Some(start) = contents[cursor..].find('<') {
        let start = cursor + start;
        let Some(end) = contents[start..].find('>') else {
            break;
        };
        let end = start + end + 1;
        tags.push(&contents[start..end]);
        cursor = end;
    }
    tags
}

fn parse_tag(tag: &str) -> Option<(String, HashMap<String, String>)> {
    let inner = tag
        .trim()
        .strip_prefix('<')?
        .strip_suffix('>')?
        .trim()
        .trim_end_matches('/')
        .trim();
    if inner.starts_with(['/', '!', '?']) {
        return None;
    }

    let mut name_end = inner.len();
    for (index, character) in inner.char_indices() {
        if character.is_whitespace() || character == '/' {
            name_end = index;
            break;
        }
    }
    let name = inner[..name_end].to_lowercase();
    if name.is_empty() {
        return None;
    }
    let attrs = parse_attributes(&inner[name_end..]);
    Some((name, attrs))
}

fn parse_attributes(input: &str) -> HashMap<String, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut attrs = HashMap::new();
    let mut index = 0;

    while index < chars.len() {
        while index < chars.len() && (chars[index].is_whitespace() || chars[index] == '/') {
            index += 1;
        }
        if index >= chars.len() {
            break;
        }

        let name_start = index;
        while index < chars.len()
            && !chars[index].is_whitespace()
            && chars[index] != '='
            && chars[index] != '/'
        {
            index += 1;
        }
        let name: String = chars[name_start..index].iter().collect();
        if name.is_empty() {
            break;
        }

        while index < chars.len() && chars[index].is_whitespace() {
            index += 1;
        }

        let mut value = String::new();
        if index < chars.len() && chars[index] == '=' {
            index += 1;
            while index < chars.len() && chars[index].is_whitespace() {
                index += 1;
            }

            if index < chars.len() && (chars[index] == '"' || chars[index] == '\'') {
                let quote = chars[index];
                index += 1;
                let value_start = index;
                while index < chars.len() && chars[index] != quote {
                    index += 1;
                }
                value = chars[value_start..index].iter().collect();
                if index < chars.len() {
                    index += 1;
                }
            } else {
                let value_start = index;
                while index < chars.len() && !chars[index].is_whitespace() && chars[index] != '/' {
                    index += 1;
                }
                value = chars[value_start..index].iter().collect();
            }
        }

        attrs.insert(name.to_lowercase(), html_unescape(&value));
    }

    attrs
}

fn extract_title(contents: &str) -> Option<String> {
    let lower = contents.to_lowercase();
    let title_start = lower.find("<title")?;
    let open_end = lower[title_start..].find('>')? + title_start + 1;
    let close_start = lower[open_end..].find("</title>")? + open_end;
    Some(normalize_text(&html_unescape(
        &contents[open_end..close_start],
    )))
}

fn extract_xml_elements(contents: &str, element_name: &str) -> Vec<String> {
    let lower = contents.to_lowercase();
    let open = format!("<{}>", element_name.to_lowercase());
    let close = format!("</{}>", element_name.to_lowercase());
    let mut values = Vec::new();
    let mut cursor = 0;

    while let Some(start) = lower[cursor..].find(&open) {
        let value_start = cursor + start + open.len();
        let Some(end) = lower[value_start..].find(&close) else {
            break;
        };
        let value_end = value_start + end;
        values.push(normalize_text(&html_unescape(
            &contents[value_start..value_end],
        )));
        cursor = value_end + close.len();
    }

    values
}

fn robots_has_sitemap(contents: &str) -> bool {
    contents
        .lines()
        .any(|line| line.trim_start().to_lowercase().starts_with("sitemap:"))
}

fn parse_robots_disallow_rules(contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let lower = trimmed.to_lowercase();
            lower
                .strip_prefix("disallow:")
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|_| trimmed[9..].trim().to_string())
        })
        .collect()
}

fn robots_disallows(relative_path: &str, rules: &[String]) -> bool {
    let page_url_path = url_path_for_relative(relative_path);
    rules.iter().any(|rule| {
        let normalized = rule.trim();
        normalized == "/" || (!normalized.is_empty() && page_url_path.starts_with(normalized))
    })
}

fn parse_vercelignore(contents: &str) -> Vec<IgnorePattern> {
    contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }
            let body = trimmed.trim_start_matches('!');
            let directory_only = body.ends_with('/');
            let normalized = normalize_relative_path(body.trim_start_matches('/'));
            Some(IgnorePattern {
                original: trimmed.to_string(),
                normalized: normalized.trim_end_matches('/').to_string(),
                directory_only,
                supported: is_supported_vercelignore_pattern(trimmed, body),
            })
        })
        .filter(|pattern| !pattern.normalized.is_empty())
        .collect()
}

fn vercelignore_evidence(relative_path: &str, patterns: &[IgnorePattern]) -> Vec<String> {
    patterns
        .iter()
        .filter(|pattern| pattern.supported)
        .filter(|pattern| vercelignore_matches(relative_path, pattern))
        .map(|pattern| format!(".vercelignore matches `{}`", pattern.original))
        .collect()
}

fn unsupported_vercelignore_evidence(
    relative_path: &str,
    patterns: &[IgnorePattern],
) -> Vec<String> {
    patterns
        .iter()
        .filter(|pattern| !pattern.supported)
        .filter(|pattern| unsupported_vercelignore_pattern_relates(relative_path, pattern))
        .map(|pattern| {
            format!(
                "unsupported .vercelignore pattern requires review: `{}`",
                pattern.original
            )
        })
        .collect()
}

fn is_supported_vercelignore_pattern(original: &str, body: &str) -> bool {
    !original.starts_with('!')
        && !body.starts_with('/')
        && !body.contains("**")
        && !body
            .chars()
            .any(|character| matches!(character, '*' | '?' | '[' | ']' | '\\'))
}

fn vercelignore_matches(relative_path: &str, pattern: &IgnorePattern) -> bool {
    let relative_path = normalize_relative_path(relative_path);
    if pattern.directory_only {
        return relative_path == pattern.normalized
            || relative_path.starts_with(&format!("{}/", pattern.normalized));
    }
    relative_path == pattern.normalized
        || relative_path.starts_with(&format!("{}/", pattern.normalized))
}

fn unsupported_vercelignore_pattern_relates(relative_path: &str, pattern: &IgnorePattern) -> bool {
    let relative_path = normalize_relative_path(relative_path);
    let prefix = pattern
        .normalized
        .split(['*', '?', '[', ']', '\\'])
        .next()
        .unwrap_or("")
        .trim_end_matches('/');
    prefix.is_empty()
        || relative_path == prefix
        || relative_path.starts_with(&format!("{prefix}/"))
        || relative_path.starts_with(prefix)
}

fn sitemap_url_to_relative_path(url: &str) -> Option<String> {
    let without_fragment = url.split('#').next().unwrap_or(url);
    let without_query = without_fragment
        .split('?')
        .next()
        .unwrap_or(without_fragment);
    let lower = without_query.to_lowercase();
    if !lower.starts_with(BASE_URL) {
        return None;
    }

    let path = &without_query[BASE_URL.len()..];
    if path.is_empty() {
        Some("index.html".to_string())
    } else if path.ends_with('/') {
        Some(format!("{}index.html", normalize_relative_path(path)))
    } else {
        Some(normalize_relative_path(path))
    }
}

fn url_path_for_relative(relative_path: &str) -> String {
    if relative_path.eq_ignore_ascii_case("index.html") {
        "/".to_string()
    } else {
        format!("/{}", normalize_relative_path(relative_path))
    }
}

fn document_evidence(root: &Path, relative_path: &str) -> Vec<String> {
    let mut evidence = Vec::new();
    collect_document_evidence(root, root, relative_path, 0, &mut evidence);
    evidence.sort();
    evidence.dedup();
    evidence.truncate(3);
    evidence
}

fn collect_document_evidence(
    root: &Path,
    directory: &Path,
    relative_path: &str,
    depth: usize,
    evidence: &mut Vec<String>,
) {
    if depth > MAX_SCAN_DEPTH || evidence.len() >= 3 {
        return;
    }
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };

    for entry in entries.flatten() {
        if evidence.len() >= 3 {
            return;
        }
        let path = entry.path();
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if is_reparse_point(&metadata) {
            continue;
        }
        if metadata.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if should_skip_scan_directory(&name) {
                continue;
            }
            collect_document_evidence(root, &path, relative_path, depth + 1, evidence);
        } else if metadata.is_file()
            && path
                .extension()
                .is_some_and(|value| value.eq_ignore_ascii_case("md"))
        {
            collect_document_evidence_from_file(root, &path, relative_path, evidence);
        }
    }
}

fn collect_document_evidence_from_file(
    root: &Path,
    path: &Path,
    relative_path: &str,
    evidence: &mut Vec<String>,
) {
    let Ok(contents) = read_limited_utf8(path, MAX_TEXT_BYTES) else {
        return;
    };
    let relative_lower = relative_path.to_lowercase();
    for line in contents.lines() {
        if evidence.len() >= 3 {
            return;
        }
        let lower = line.to_lowercase();
        if lower.contains(&relative_lower) && contains_private_or_exclusion_keyword(&lower) {
            evidence.push(format!(
                "documentation evidence in {}",
                relative_path_from_root(root, path)
            ));
            return;
        }
    }
}

fn contains_private_or_exclusion_keyword(line: &str) -> bool {
    [
        "exclude",
        "excluded",
        "private",
        "prototype",
        "admin",
        "ignore",
        "not deploy",
    ]
    .iter()
    .any(|keyword| line.contains(keyword))
}

fn validate_repository_root(input: &str) -> Result<PathBuf, SiteStatusError> {
    if input.trim().is_empty() {
        return Err(error(
            "REPOSITORY_ROOT_REQUIRED",
            "Repository root path is required.",
        ));
    }

    let selected = PathBuf::from(input.trim());
    let metadata = fs::symlink_metadata(&selected)
        .map_err(|_| error("ROOT_ACCESS_DENIED", "Repository root is not readable."))?;
    if !metadata.is_dir() || is_reparse_point(&metadata) {
        return Err(error(
            "INVALID_REPOSITORY_ROOT",
            "Repository root must be a regular directory.",
        ));
    }

    let root = fs::canonicalize(&selected).map_err(|_| {
        error(
            "ROOT_ACCESS_DENIED",
            "Repository root cannot be canonicalized.",
        )
    })?;
    if !is_valid_korea_inside_root(&root) {
        return Err(error(
            "INVALID_REPOSITORY_ROOT",
            "Selected path is not a Korea Inside repository root.",
        ));
    }
    Ok(root)
}

fn is_valid_korea_inside_root(root: &Path) -> bool {
    is_readable_regular_file(&root.join("PROJECT.md"))
        && is_readable_regular_file(&root.join("AGENTS.md"))
        && is_readable_regular_file(&root.join("index.html"))
        && is_readable_regular_file(&root.join("style.css"))
        && is_readable_regular_file(&root.join("common.js"))
        && is_valid_git_metadata(&root.join(".git"))
        && is_readable_regular_file(
            &root
                .join("korea-inside-admin")
                .join("src-tauri")
                .join("tauri.conf.json"),
        )
}

fn read_limited_utf8(path: &Path, max_bytes: u64) -> io::Result<String> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || is_reparse_point(&metadata) || metadata.len() > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "file is invalid or too large",
        ));
    }

    let canonical = fs::canonicalize(path)?;
    if canonical != path && fs::symlink_metadata(&canonical).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "canonical file is not readable",
        ));
    }

    let file = fs::File::open(path)?;
    let mut contents = String::new();
    file.take(max_bytes + 1).read_to_string(&mut contents)?;
    if contents.len() as u64 > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "file is too large",
        ));
    }
    Ok(contents)
}

fn checked_child_path(root: &Path, relative_path: &str) -> Result<PathBuf, SiteStatusError> {
    let normalized = normalize_relative_path(relative_path);
    if normalized.is_empty()
        || normalized.starts_with("../")
        || normalized.contains("/../")
        || normalized == ".."
        || Path::new(&normalized).is_absolute()
    {
        return Err(error(
            "PATH_OUTSIDE_REPOSITORY",
            "Relative path escapes the repository root.",
        ));
    }

    let candidate = root.join(normalized);
    let parent = candidate.parent().unwrap_or(root);
    if parent.exists() {
        let canonical_parent = fs::canonicalize(parent)
            .map_err(|_| error("PATH_ACCESS_DENIED", "Path parent is not readable."))?;
        if !canonical_parent.starts_with(root) {
            return Err(error(
                "PATH_OUTSIDE_REPOSITORY",
                "Resolved path escapes the repository root.",
            ));
        }
    }
    Ok(candidate)
}

fn canonical_inside(root: &Path, path: &Path) -> bool {
    fs::canonicalize(path).is_ok_and(|canonical| canonical.starts_with(root))
}

fn is_readable_regular_file(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && !is_reparse_point(&metadata))
        && fs::File::open(path).is_ok()
}

fn is_valid_git_metadata(path: &Path) -> bool {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return false;
    };
    if is_reparse_point(&metadata) {
        return false;
    }
    if metadata.is_dir() {
        return true;
    }
    if !metadata.is_file() || metadata.len() > MAX_TEXT_BYTES {
        return false;
    }
    read_limited_utf8(path, MAX_TEXT_BYTES).is_ok_and(|contents| {
        contents
            .lines()
            .next()
            .is_some_and(|line| line.starts_with("gitdir:"))
    })
}

fn is_reparse_point(metadata: &fs::Metadata) -> bool {
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

fn is_html_file(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("html"))
}

fn should_skip_scan_directory(name: &str) -> bool {
    EXCLUDED_SCAN_DIRECTORIES
        .iter()
        .any(|excluded| name.eq_ignore_ascii_case(excluded))
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(relative_path_from_relative)
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn relative_path_from_root(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(relative_path_from_relative)
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn relative_path_from_relative(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn normalize_relative_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}

fn normalize_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_blank_option(value: Option<&str>) -> bool {
    value.map(str::is_empty).unwrap_or(true)
}

fn html_unescape(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn sorted_limited(values: HashSet<String>) -> Vec<String> {
    let mut values: Vec<String> = values.into_iter().collect();
    values.sort();
    values.truncate(MAX_GLOBAL_LIST_ITEMS);
    values
}

fn push_limited<T>(values: &mut Vec<T>, value: T) {
    if values.len() < MAX_GLOBAL_LIST_ITEMS {
        values.push(value);
    }
}

#[derive(Debug)]
struct SiteStatusError {
    code: &'static str,
    message: String,
}

fn error(code: &'static str, message: &str) -> SiteStatusError {
    SiteStatusError {
        code,
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parses_complete_html_with_flexible_attribute_order_and_case() {
        let html = r#"
            <HTML>
              <head>
                <TITLE>Korea Inside &amp; Travel</TITLE>
                <meta content='Practical Korea guide' name='description'>
                <link href="https://www.getkoreainside.com/" rel="canonical">
              </head>
              <body><H1>How Korea Works</H1></body>
            </HTML>
        "#;
        let metadata = analyze_html(html);

        assert_eq!(metadata.title.as_deref(), Some("Korea Inside & Travel"));
        assert_eq!(
            metadata.meta_description.as_deref(),
            Some("Practical Korea guide")
        );
        assert_eq!(
            metadata.canonical.as_deref(),
            Some("https://www.getkoreainside.com/")
        );
        assert_eq!(metadata.h1_count, 1);
        assert!(!metadata.robots_noindex);
    }

    #[test]
    fn reports_missing_title() {
        let page = analyze_sample_page("<h1>Guide</h1><meta name=\"description\" content=\"Info\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/test.html\">");

        assert!(page.issues.contains(&"missing_title".to_string()));
    }

    #[test]
    fn reports_missing_description() {
        let page = analyze_sample_page("<title>Guide</title><h1>Guide</h1><link rel=\"canonical\" href=\"https://www.getkoreainside.com/test.html\">");

        assert!(page
            .issues
            .contains(&"missing_meta_description".to_string()));
    }

    #[test]
    fn reports_missing_canonical() {
        let page = analyze_sample_page(
            "<title>Guide</title><meta name=\"description\" content=\"Info\"><h1>Guide</h1>",
        );

        assert!(page.issues.contains(&"missing_canonical".to_string()));
    }

    #[test]
    fn reports_missing_h1() {
        let page = analyze_sample_page("<title>Guide</title><meta name=\"description\" content=\"Info\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/test.html\">");

        assert!(page.issues.contains(&"missing_h1".to_string()));
    }

    #[test]
    fn reports_multiple_h1() {
        let page = analyze_sample_page("<title>Guide</title><meta name=\"description\" content=\"Info\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/test.html\"><h1>One</h1><h1>Two</h1>");

        assert!(page.issues.contains(&"multiple_h1".to_string()));
    }

    #[test]
    fn reports_noindex_page_as_review_required() {
        let page = analyze_sample_page("<title>Guide</title><meta name=\"description\" content=\"Info\"><meta name=\"robots\" content=\"noindex, nofollow\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/test.html\"><h1>Guide</h1>");

        assert!(page.robots_noindex);
        assert_eq!(page.page_classification, PageClassification::ReviewRequired);
        assert!(page.issues.contains(&"robots_noindex".to_string()));
    }

    #[test]
    fn detects_sitemap_included_and_missing_pages() {
        let root = test_repo_root();
        write_file(
            &root.join("sitemap.xml"),
            "<urlset><url><loc>https://www.getkoreainside.com/</loc></url><url><loc>https://www.getkoreainside.com/missing.html</loc></url></urlset>",
        );
        write_file(
            &root.join("robots.txt"),
            "User-agent: *\nAllow: /\nSitemap: https://www.getkoreainside.com/sitemap.xml\n",
        );
        write_file(&root.join(".vercelignore"), "");
        write_file(&root.join("vercel.json"), "{}");

        let report = build_site_status_report(&root).unwrap();
        let index = report
            .pages
            .iter()
            .find(|page| page.relative_path == "index.html")
            .unwrap();

        assert!(index.sitemap_included);
        assert!(report
            .global
            .sitemap_urls_without_local_file
            .contains(&"missing.html".to_string()));
    }

    #[test]
    fn handles_invalid_utf8_without_panicking() {
        let root = test_repo_root();
        fs::write(root.join("broken.html"), [0xff, 0xfe, 0xfd]).unwrap();

        let report = build_site_status_report(&root).unwrap();
        let broken = report
            .pages
            .iter()
            .find(|page| page.relative_path == "broken.html")
            .unwrap();

        assert!(broken
            .issues
            .contains(&"html_invalid_or_too_large".to_string()));
        assert_eq!(report.summary.error_count, 1);
    }

    #[test]
    fn blocks_paths_that_escape_repository_root() {
        let root = test_repo_root();

        assert_eq!(
            checked_child_path(&root, "../outside.html")
                .unwrap_err()
                .code,
            "PATH_OUTSIDE_REPOSITORY"
        );
    }

    #[test]
    fn classifies_vercelignored_pages_as_excluded() {
        let root = test_repo_root();
        write_file(&root.join(".vercelignore"), "admin/\n");
        fs::create_dir_all(root.join("admin")).unwrap();
        write_file(
            &root.join("admin").join("tool.html"),
            "<title>Tool</title><meta name=\"description\" content=\"Tool\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/admin/tool.html\"><h1>Tool</h1>",
        );

        let report = build_site_status_report(&root).unwrap();
        let page = report
            .pages
            .iter()
            .find(|page| page.relative_path == "admin/tool.html")
            .unwrap();

        assert_eq!(page.page_classification, PageClassification::Excluded);
        assert!(page
            .issues
            .contains(&"excluded_by_vercelignore".to_string()));
    }

    #[test]
    fn negation_pattern_requires_review_without_excluding_page() {
        let root = test_repo_root();
        write_file(&root.join(".vercelignore"), "admin/\n!admin/tool.html\n");
        fs::create_dir_all(root.join("admin")).unwrap();
        write_file(
            &root.join("admin").join("tool.html"),
            "<title>Tool</title><meta name=\"description\" content=\"Tool\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/admin/tool.html\"><h1>Tool</h1>",
        );

        let report = build_site_status_report(&root).unwrap();
        let page = report_page(&report, "admin/tool.html");

        assert_eq!(page.page_classification, PageClassification::ReviewRequired);
        assert!(page
            .issues
            .contains(&"unsupported_vercelignore_pattern_requires_review".to_string()));
        assert!(!page
            .issues
            .contains(&"excluded_by_vercelignore".to_string()));
    }

    #[test]
    fn globstar_pattern_requires_review_without_excluding_page() {
        let root = test_repo_root();
        write_file(&root.join(".vercelignore"), "admin/**\n");
        fs::create_dir_all(root.join("admin")).unwrap();
        write_file(
            &root.join("admin").join("tool.html"),
            "<title>Tool</title><meta name=\"description\" content=\"Tool\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/admin/tool.html\"><h1>Tool</h1>",
        );

        let report = build_site_status_report(&root).unwrap();
        let page = report_page(&report, "admin/tool.html");

        assert_eq!(page.page_classification, PageClassification::ReviewRequired);
        assert!(page
            .issues
            .contains(&"unsupported_vercelignore_pattern_requires_review".to_string()));
        assert!(!page
            .issues
            .contains(&"excluded_by_vercelignore".to_string()));
    }

    #[test]
    fn simple_vercelignore_file_pattern_still_excludes_page() {
        let root = test_repo_root();
        write_file(&root.join(".vercelignore"), "private.html\n");
        write_file(
            &root.join("private.html"),
            "<title>Private</title><meta name=\"description\" content=\"Private\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/private.html\"><h1>Private</h1>",
        );

        let report = build_site_status_report(&root).unwrap();
        let page = report_page(&report, "private.html");

        assert_eq!(page.page_classification, PageClassification::Excluded);
        assert!(page
            .issues
            .contains(&"excluded_by_vercelignore".to_string()));
    }

    #[test]
    fn unsupported_character_class_pattern_returns_review_warning() {
        let root = test_repo_root();
        write_file(&root.join(".vercelignore"), "draft[0-9].html\n");
        write_file(
            &root.join("draft1.html"),
            "<title>Draft</title><meta name=\"description\" content=\"Draft\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/draft1.html\"><h1>Draft</h1>",
        );

        let report = build_site_status_report(&root).unwrap();
        let page = report_page(&report, "draft1.html");
        let evidence = report
            .global
            .deployment_exclusion_evidence
            .iter()
            .find(|item| item.relative_path == "draft1.html")
            .unwrap();

        assert_eq!(page.page_classification, PageClassification::ReviewRequired);
        assert!(page
            .issues
            .contains(&"unsupported_vercelignore_pattern_requires_review".to_string()));
        assert!(evidence
            .evidence
            .iter()
            .any(|item| item.contains("unsupported .vercelignore pattern requires review")));
    }

    fn report_page<'a>(report: &'a SiteStatusReport, relative_path: &str) -> &'a HtmlPageStatus {
        report
            .pages
            .iter()
            .find(|page| page.relative_path == relative_path)
            .unwrap()
    }

    fn analyze_sample_page(html: &str) -> HtmlPageStatus {
        let root = test_repo_root();
        write_file(&root.join("test.html"), html);
        let inputs = collect_site_inputs(&root);
        analyze_html_page(&root, &root.join("test.html"), &inputs)
    }

    fn test_repo_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "korea-inside-site-status-test-{}-{stamp}",
            std::process::id()
        ));

        fs::create_dir_all(root.join(".git")).unwrap();
        fs::create_dir_all(root.join("korea-inside-admin").join("src-tauri")).unwrap();
        write_file(&root.join("PROJECT.md"), "# Korea Inside\n");
        write_file(&root.join("AGENTS.md"), "# AGENTS\n");
        write_file(&root.join("style.css"), "");
        write_file(&root.join("common.js"), "");
        write_file(&root.join("vercel.json"), "{}");
        write_file(&root.join("robots.txt"), "User-agent: *\nAllow: /\n");
        write_file(&root.join(".vercelignore"), "");
        write_file(
            &root.join("sitemap.xml"),
            "<urlset><url><loc>https://www.getkoreainside.com/</loc></url></urlset>",
        );
        write_file(
            &root.join("index.html"),
            "<title>Home</title><meta name=\"description\" content=\"Home\"><link rel=\"canonical\" href=\"https://www.getkoreainside.com/\"><h1>Home</h1>",
        );
        write_file(
            &root
                .join("korea-inside-admin")
                .join("src-tauri")
                .join("tauri.conf.json"),
            "{\"identifier\":\"com.getkoreainside.admin\"}",
        );

        fs::canonicalize(root).unwrap()
    }

    fn write_file(path: &Path, contents: &str) {
        fs::write(path, contents).unwrap();
    }
}
