//  hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use configparser::ini::Ini;
use std::{path::Path, time};
use std::error::Error;
use walkdir::WalkDir;


fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new();
    config.load("config.ini")?;

    let path_str = config.get("system", "path").expect("The Path to the Folder does not set.");
    let path = Path::new(&path_str);

    let period_str = config.get("system", "period").expect("The Period does not set.");
    let period: u64 = period_str.parse()?;

    let mode = config.get("system", "mode");

    let ext_str = config.get("extensions", "excluded").expect("Extensions not found");
    let exts: Vec<&str> =
        ext_str
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    loop {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|f| f.path().is_file()
                && (f.path().extension().is_none() || !exts.contains(&f.path()
                .extension().unwrap().to_str().unwrap()))
            )
            {
                std::fs::remove_file(entry.path())?;
        }
        if mode == Some("once".to_owned()) { break; }
        std::thread::sleep(time::Duration::from_secs(period));
    }

    Ok(())
}
