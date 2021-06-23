#[cfg(test)]
mod tests;

use home;
use std::{fmt, io};

pub fn get_steam_compatdata_path() -> Result<String, fmt::Error> {
    return match home::home_dir() {
        Some(path) => {
            let home_folder = path.display();
            Ok(format!(
                "{}/.steam/steam/compatdata",
                home_folder.to_string()
            ))
        }
        None => panic!("Couldn't retrieve home directory!"),
    };
}

pub fn create_appid_path(appid: u32) -> io::Result<()> {
    return std::fs::create_dir_all(get_appid_path(appid).unwrap());
}
pub fn create_appid_pfx_path(appid: u32) -> io::Result<()> {
    return std::fs::create_dir_all(get_appid_pfx_path(appid).unwrap());
}
pub fn remove_appid_path(appid: u32) -> io::Result<()> {
    return std::fs::remove_dir_all(get_appid_path(appid).unwrap());
}
pub fn remove_appid_pfx_path(appid: u32) -> io::Result<()> {
    return std::fs::remove_dir_all(get_appid_pfx_path(appid).unwrap());
}

pub fn get_appid_pfx_path(appid: u32) -> Result<String, fmt::Error> {
    Ok(format!(
        "{}/{}/pfx",
        get_steam_compatdata_path().unwrap(),
        appid
    ))
}
pub fn get_appid_path(appid: u32) -> Result<String, fmt::Error> {
    Ok(format!(
        "{}/{}",
        get_steam_compatdata_path().unwrap(),
        appid
    ))
}

pub struct SymlinkResult {
    pub appid_1_path: String,
    pub appid_2_path: String,
}

pub fn link_appids(appid_1: u32, appid_2: u32) -> SymlinkResult {
    create_appid_path(appid_2).unwrap();

    let appid_1_path = get_appid_pfx_path(appid_1).unwrap();
    let appid_2_path = get_appid_pfx_path(appid_2).unwrap();
    std::os::unix::fs::symlink(&appid_1_path, &appid_2_path).unwrap();

    return SymlinkResult {
        appid_1_path,
        appid_2_path,
    };
}
