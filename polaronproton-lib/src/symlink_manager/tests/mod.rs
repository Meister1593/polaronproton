use std::u32;

use tempfile::TempDir;

use crate::symlink_manager::*;

#[test]
fn link_appids_test() {
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_some_path = Some(temp_dir.path().display().to_string());

    let appid_1: u32 = 0;
    let appid_2: u32 = 1;
    create_appid_pfx_path(appid_1, &temp_dir_some_path).unwrap();

    link_appids(appid_1, appid_2, &temp_dir_some_path);
    // std::thread::sleep(Duration::from_millis(1000));
    let appid_2_path = get_appid_pfx_path(appid_2, &temp_dir_some_path).unwrap();
    let appid_2_path_exists = Path::new(&appid_2_path).exists();
    assert!(appid_2_path_exists);
}

#[test]
fn create_appid_backup_if_needed_test() {
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_some_path = Some(temp_dir.path().display().to_string());

    let appid: u32 = 2;
    let appid_backup_path = format!(
        "{}.backup",
        get_appid_pfx_path(appid, &temp_dir_some_path).unwrap()
    );
    create_appid_pfx_path(appid, &temp_dir_some_path).unwrap();

    create_appid_backup_if_needed(appid, &temp_dir_some_path).unwrap();
    let appid_backup_exists = Path::new(&appid_backup_path).exists();
    assert!(appid_backup_exists);
}
