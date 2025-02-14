// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod desktop_manager; // 引入模块
mod desktop_restore;
mod desktop_windows;
fn main() {
    println!("里层main.rs");
    // get_icons();
    // windows_js_lib::run();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            desktop_windows::handle_get_categorized_files,
            desktop_restore::restore_files_to_desktop,
            desktop_manager::get_categorized_files,
            desktop_manager::organize_desktop
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}   
// fn get_icons() {

//     println!("开始获取桌面icon");
//     fn get_desktop_files() -> Vec<PathBuf> {
//         if let Some(desktop_path) = dirs::desktop_dir() {
//             match fs::read_dir(desktop_path) {
//                 Ok(entries) => entries.filter_map(|entry| entry.ok().map(|e| e.path())).collect(),
//                 Err(e) => {
//                     println!("无法读取桌面目录: {}", e);
//                     Vec::new()
//                 }
//             }
//         } else {
//             println!("无法获取桌面路径");
//             Vec::new()
//         }
//     }

//     let files = get_desktop_files();
//     println!("桌面文件: {:?}", files);
// }