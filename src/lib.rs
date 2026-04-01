#![allow(dead_code)]

pub mod config;
pub use config::Config;

use std::error::Error;
use std::fs::{self, DirEntry};

pub fn index_videos(config: Config) -> Result<(), Box<dyn Error>> {
    let dcim_path = config.get_dcim_path();
    if let Ok(entries) = fs::read_dir(&dcim_path) {
        println!("Here are the files:");
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("We've got a folder: {:?}", &path);
                todo!();
            } else {
                process_video(&entry);
            }
        }
    }

    Ok(())
}

fn process_video(file: &DirEntry) {
    println!("Processing file: {:?}", &file.path());
}
