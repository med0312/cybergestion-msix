use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{command, generate_context, generate_handler, Builder, Manager, State};

#[command]
fn get_app_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
}

#[command]
async fn pick_folder() -> Option<PathBuf> {
    FileDialogBuilder::new().pick_folder()
}

#[command]
async fn read_text_file(path: PathBuf) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[command]
async fn write_text_file(path: PathBuf, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![
            get_app_dir,
            pick_folder,
            read_text_file,
            write_text_file
        ])
        .run(generate_context!())
        .expect("erreur lors du lancement de Tauri");
}