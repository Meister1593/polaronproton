use std::env;

use polaronproton_lib::{common, symlink_manager};

enum Argument {
    Help,
    Linking,
    Version,
    None,
    Unknown,
}

struct CliParser;
trait ArgumentParse {
    fn parse_type(&self, args: &Vec<String>) -> Argument;
    fn process(&self, args: &Vec<String>);
}

impl ArgumentParse for CliParser {
    fn parse_type(&self, args: &Vec<String>) -> Argument {
        match args.len() {
            1 => return Argument::None,
            2 => match args[1].as_ref() {
                "-h" | "--help" => return Argument::Help,
                "-v" | "--version" => return Argument::Version,
                _ => return Argument::Unknown,
            },
            3 => {
                let arg_1 = &args[1];
                match arg_1.parse::<u32>() {
                    Err(_) => return Argument::Unknown,
                    _ => (),
                };
                let arg_2 = &args[2];
                match arg_2.parse::<u32>() {
                    Err(_) => return Argument::Unknown,
                    _ => (),
                }
                return Argument::Linking;
            }
            _ => Argument::Unknown,
        }
    }
    fn process(&self, args: &Vec<String>) {
        match self.parse_type(&args) {
            Argument::Help => {
                println!("Usage: polaronproton [OPTION]... [APPID_1] [APPID_2]");
                println!("Make symlinks in proton compatdata folder, appid_1 is source folder and appid_2 is linking folder\n");
                println!("\t-h\t--help\tShow this help.");
            }
            Argument::Linking => {
                let appid_1 = args[1].parse::<u32>().unwrap();
                let appid_2 = args[2].parse::<u32>().unwrap();
                let result = symlink_manager::link_appids(appid_1, appid_2, &Option::None);
                if result.is_backup_created {
                    println!("Created backup for {} appid pfx", appid_2);
                }
                println!(
                    "Linked appid 1 with path '{}'\nTo appid 2 with path '{}'",
                    result.appid_1_path, result.appid_2_path
                );
            }
            Argument::Version => {
                println!("polaronproton {}", common::VERSION)
            }
            Argument::None => {
                println!("No arguments to process");
            }
            Argument::Unknown => {
                println!("Unknown arguments. Try using '-h' or '--help' for usage info.");
            }
        }
    }
}
pub fn begin() {
    let args: Vec<String> = env::args().collect();
    let parser = CliParser {};
    parser.process(&args);
}
