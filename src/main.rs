// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use configparser::ini::Ini;
use std::{path::Path, time};
use walkdir::WalkDir;


fn main() {
    let mut config = Ini::new();
    config.load("config.ini").unwrap();

    let path_str = config.get("system", "path").expect("The Path to the Folder does not set.");
    let path = Path::new(&path_str);

    let period_str = config.get("system", "period").expect("The Period does not set.");
    let period: u64 = period_str.parse().unwrap();

    let ext_str = config.get("extensions", "excluded").expect("Extensions not found");
    let exts: Box<Vec<&str>> = Box::new(ext_str.split(",").collect());

    loop {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|f| f.path().is_file() && !exts.contains(&f.path().extension().unwrap().to_str().unwrap()))
            {
                std::fs::remove_file(entry.path()).unwrap();
        }
        std::thread::sleep(time::Duration::from_secs(period));
    }
}
