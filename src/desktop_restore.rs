use regex::Regex;
use std::fs;
use std::path::Path;

/// 递归查找桌面文件夹，将 YYYY-MM 文件夹中的文件移回桌面
#[tauri::command]
pub fn restore_files_to_desktop() {  // ✅ 确保是 pub
    let desktop_path = dirs::desktop_dir().expect("无法获取桌面路径");
    move_files_from_date_folders(&desktop_path, &desktop_path);
}

/// 递归遍历文件夹，找到 YYYY-MM 目录，并将文件移动到桌面
fn move_files_from_date_folders(current_path: &Path, desktop: &Path) {
    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap().to_string_lossy().to_string();

                if is_date_folder(&folder_name) {
                    move_all_files_to_desktop(&path, desktop);
                } else {
                    move_files_from_date_folders(&path, desktop);
                }
            }
        }
    }
}

/// 移动文件夹内的所有文件到桌面，并删除空文件夹
fn move_all_files_to_desktop(folder: &Path, desktop: &Path) {
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                let new_path = desktop.join(file_path.file_name().unwrap());
                if let Err(e) = fs::rename(&file_path, &new_path) {
                    eprintln!("移动 {:?} 失败: {:?}", file_path, e);
                }
            }
        }
    }

    if folder.read_dir().map(|mut i| i.next().is_none()).unwrap_or(false) {
        if let Err(e) = fs::remove_dir(folder) {
            eprintln!("删除空文件夹 {:?} 失败: {:?}", folder, e);
        }
    }
}

/// 判断文件夹名称是否为 YYYY-MM 格式
fn is_date_folder(folder_name: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}$").unwrap();
    re.is_match(folder_name)
}
