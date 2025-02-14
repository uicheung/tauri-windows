use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc, Datelike};
use serde::Serialize;
use dirs;

/// 结构体：存储文件信息
#[derive(Serialize)]
pub struct FileInfo {
    name: String,
    path: String,
    category: String,
    date: String,
}

/// 获取桌面路径
fn get_desktop_path() -> PathBuf {
    dirs::desktop_dir().unwrap()
}

/// 获取桌面所有文件
fn get_desktop_files() -> Vec<PathBuf> {
    let desktop_path = get_desktop_path();
    fs::read_dir(desktop_path)
        .unwrap()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect()
}

/// 识别文件类型
fn get_file_category(file_path: &PathBuf) -> &'static str {
    if let Some(ext) = file_path.extension() {
        match ext.to_str().unwrap_or("").to_lowercase().as_str() {
            "jpg" | "png" | "gif" | "bmp" | "svg" => "Images",
            "mp4" | "mkv" | "avi" | "mov" => "Videos",
            "mp3" | "wav" | "flac" => "Music",
            "exe" | "msi" => "Software",
            "pdf" | "docx" | "txt" | "xlsx" => "Documents",
            _ => "Others",
        }
    } else {
        "Others"
    }
}

/// 获取文件的修改日期（格式：YYYY-MM）
fn get_file_date(file_path: &PathBuf) -> String {
    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified) = metadata.modified() {
            let datetime: DateTime<Utc> = modified.into();
            return format!("{}-{:02}", datetime.year(), datetime.month());
        }
    }
    "Unknown".to_string()
}

/// 获取分类后的文件列表
#[tauri::command]
pub fn handle_get_categorized_files() -> Vec<FileInfo> {
    let files = get_desktop_files();
    let mut categorized_files = Vec::new();

    for file in files {
        if file.is_file() {
            let category = get_file_category(&file);
            let date = get_file_date(&file);
            categorized_files.push(FileInfo {
                name: file.file_name().unwrap().to_string_lossy().into_owned(),
                path: file.to_string_lossy().into_owned(),
                category: category.to_string(),
                date,
            });
        }
    }
    categorized_files
}