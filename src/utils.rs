use dirs::home_dir;
use std::fs;

pub const RODO_DB_FILENAME: &str = ".rododb";

// 获取 db 文件路径
pub fn get_db_file_path() -> std::path::PathBuf {
    home_dir()
        .map(|it| it.join(RODO_DB_FILENAME))
        .unwrap_or_default()
}

/// 检查 db 文件是否存在
pub fn db_exists() -> bool {
    let dir = get_db_file_path();
    fs::metadata(dir).is_ok()
}

// 创建 db 文件
pub fn create_db_file() -> std::io::Result<()> {
    let dir = get_db_file_path();
    fs::File::create(dir)?;
    Ok(())
}

// 检查db文件是否存在，不存在就创建
pub fn check_db_file() -> std::io::Result<()> {
    if !db_exists() {
        create_db_file()?;
    }
    Ok(())
}
