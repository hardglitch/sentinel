//  hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use configparser::ini::Ini;
use std::path::Path;
use std::time::Duration;
use walkdir::WalkDir;

fn main() -> Result<(), String> {
    let mut config = Ini::new();
    config.load("config.ini")?;

    let p = config.get("system", "path").unwrap_or("./".to_owned());
    let path = Path::new(&p);

    let p = config.get("system", "period").unwrap_or_default();
    let period = p.parse::<u64>().unwrap_or_default();

    let mode = config.get("system", "mode");

    let ext = config.get("extensions", "excluded").unwrap_or_default();
    let exts = ext.split(',').map(|s| s.trim()).collect::<Vec<&str>>();

    loop {
        for entry in WalkDir::new(path).into_iter()
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
            for _ in 0..10 {
                if std::fs::remove_file(entry.path()).is_err() {
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
        if mode == Some("once".to_owned()) { break; }
        std::thread::sleep(Duration::from_secs(period));
    }
    Ok(())
}