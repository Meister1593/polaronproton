#[cfg(test)]
mod tests;

use home;
use std::{fmt, io, path::Path};

pub fn get_steam_compatdata_path(offset: &Option<String>) -> Result<String, fmt::Error> {
    return match offset {
        Some(offset) => {
            return Ok(format!("{}/.steam/steam/steamapps/compatdata", offset));
        }
        None => {
            let home_folder = home::home_dir().unwrap();
            let home_folder_path = home_folder.display();
            Ok(format!(
                "{}/.steam/steam/steamapps/compatdata",
                home_folder_path.to_string()
            ))
        }
    };
}

pub fn create_appid_path(appid: u32, offset: &Option<String>) -> io::Result<()> {
    return std::fs::create_dir_all(get_appid_path(appid, offset).unwrap());
}

pub fn create_appid_pfx_path(appid: u32, offset: &Option<String>) -> io::Result<()> {
    return std::fs::create_dir_all(get_appid_pfx_path(appid, offset).unwrap());
}

pub fn remove_appid_path(appid: u32, offset: &Option<String>) -> io::Result<()> {
    return std::fs::remove_dir_all(get_appid_path(appid, offset).unwrap());
}

pub fn remove_appid_pfx_path(appid: u32, offset: &Option<String>) -> io::Result<()> {
    return std::fs::remove_dir_all(get_appid_pfx_path(appid, offset).unwrap());
}

pub fn get_appid_pfx_path(appid: u32, offset: &Option<String>) -> Result<String, fmt::Error> {
    Ok(format!(
        "{}/{}/pfx",
        get_steam_compatdata_path(offset).unwrap(),
        appid
    ))
}

pub fn get_appid_path(appid: u32, offset: &Option<String>) -> Result<String, fmt::Error> {
    Ok(format!(
        "{}/{}",
        get_steam_compatdata_path(offset).unwrap(),
        appid
    ))
}

pub fn create_appid_backup_if_needed(
    appid: u32,
    offset: &Option<String>,
) -> Result<bool, io::Error> {
    let appid_path = get_appid_pfx_path(appid, offset).unwrap();
    let appid_new_path = format!("{}.backup", appid_path);
    if Path::new(&appid_path).exists() {
        return match std::fs::rename(&appid_path, &appid_new_path) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        };
    }
    Ok(false)
}

pub struct SymlinkResult {
    pub appid_1_path: String,
    pub appid_2_path: String,
    pub is_backup_created: bool,
}

pub fn link_appids(appid_1: u32, appid_2: u32, offset: &Option<String>) -> SymlinkResult {
    let is_backup_created = create_appid_backup_if_needed(appid_2, &offset).unwrap();
    create_appid_path(appid_2, &offset).unwrap();

    let appid_1_path = get_appid_pfx_path(appid_1, &offset).unwrap();
    let appid_2_path = get_appid_pfx_path(appid_2, &offset).unwrap();
    std::os::unix::fs::symlink(&appid_1_path, &appid_2_path).unwrap();

    return SymlinkResult {
        appid_1_path,
        appid_2_path,
        is_backup_created,
    };
}
