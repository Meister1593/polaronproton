#[cfg(test)]
pub mod symlink_manager_tests {
    use crate::symlink_manager::create_appid_pfx_path;
    use crate::symlink_manager::get_steam_compatdata_path;
    use crate::symlink_manager::link_appids;
    use crate::symlink_manager::remove_appid_path;
    use std::matches;
    use std::{fs, io, u32};

    #[test]
    fn link_test() {
        let appid_1: u32 = 0;
        let appid_2: u32 = 1;

        create_appid_pfx_path(appid_1).unwrap();

        link_appids(appid_1, appid_2).unwrap();

        let entries = fs::read_dir(format!(
            "{}/{}",
            get_steam_compatdata_path().unwrap(),
            appid_2
        ))
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

        let appid_2_pfx_path_pos = entries.iter().position(|entry| entry.ends_with("pfx"));
        assert!(matches!(appid_2_pfx_path_pos, Some(_)));
        remove_appid_path(appid_1).unwrap();
        remove_appid_path(appid_2).unwrap();
    }
}

pub mod symlink_manager {
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

    pub fn link_appids(appid_1: u32, appid_2: u32) -> io::Result<()> {
        create_appid_path(appid_2).unwrap();

        std::os::unix::fs::symlink(
            get_appid_pfx_path(appid_1).unwrap(),
            get_appid_pfx_path(appid_2).unwrap(),
        )
    }
}
