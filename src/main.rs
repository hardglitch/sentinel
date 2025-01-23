//  hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use configparser::ini::Ini;
use std::path::Path;
use std::time::Duration;
use walkdir::WalkDir;

fn main() {
    let mut config = Ini::new();
    config.load("config.ini").unwrap();

    let path_str = config.get("system", "path").unwrap_or("./".to_owned());
    let path = Path::new(&path_str);

    let period_str = config.get("system", "period").unwrap_or_default();
    let period: u64 = period_str.parse().unwrap_or_default();

    let mode = config.get("system", "mode");

    let ext_str = config.get("extensions", "excluded").unwrap_or_default();
    let exts: Vec<&str> =
        ext_str
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    loop {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|f|
                f.path().is_file()
                    &&
                (
                    match f.path().extension() {
                        Some(ext) => {
                            let s = ext.to_str().unwrap_or_default();
                            !exts.contains(&s)
                        },
                        None => true
                    }
                )
            )
        {
            while std::fs::remove_file(entry.path()).is_err() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        if mode == Some("once".to_owned()) { break; }
        std::thread::sleep(Duration::from_secs(period));
    }
}
