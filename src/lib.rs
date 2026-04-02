#![allow(dead_code)]

pub mod config;

pub use config::Config;

use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::{
    fs::{self},
    path::Path,
};

const INDEX_TMP_SUBPATH: &str = ".index_tmp";
const VALID_VIDEO_EXTENSIONS: [&str; 6] = ["mp4", "mov", "mkv", "avi", "webm", "m4v"];

pub fn index_videos(config: Config) -> Result<(), Box<dyn Error>> {
    index_videos_in_folder(config.get_dcim_path())
}

fn index_videos_in_folder(folder_path: &str) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(folder_path)?;

    // creating a temporary folder
    let index_tmp_path: PathBuf = [folder_path, INDEX_TMP_SUBPATH].iter().collect();
    std::fs::create_dir_all(&index_tmp_path)?;
    let index_tmp_path = index_tmp_path.canonicalize()?;

    println!("Here are the files:");
    for entry in entries {
        let entry = entry?;
        let path = entry.path().canonicalize()?;
        // ignore the index_tmp_path
        if path == index_tmp_path {
            continue;
        } else if path.is_dir() {
            println!("We've got a folder: {:?}", &path);
            index_videos_in_folder(path.to_str().unwrap())?;
        } else if is_video(&path) {
            process_video(&index_tmp_path, &path)?;
        } else {
            println!("Skipping: {:?}", path);
        }
    }

    Ok(())
}

fn is_video(file_path: &Path) -> bool {
    file_path
        .extension()
        .iter()
        .flat_map(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .any(|ext| VALID_VIDEO_EXTENSIONS.contains(&ext.as_str()))
}

fn process_video(index_tmp_path: &Path, video_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Processing file: {:?}", video_path);
    let input_video_path = video_path.to_str().unwrap();
    let video_file_stem = video_path.file_stem().unwrap().to_str().unwrap();
    let output_audio_path = {
        let mut path_buf = index_tmp_path.to_path_buf();
        path_buf.push(video_file_stem);
        path_buf.set_extension("wav");
        path_buf.to_str().unwrap().to_string()
    };
    extract_audio(input_video_path, &output_audio_path)?;
    Ok(())
}

fn extract_audio(input_video_path: &str, output_audio_path: &str) -> std::io::Result<()> {
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            input_video_path,
            "-ar",
            "16000", // 16kHz
            "-ac",
            "1", // mono
            "-c:a",
            "pcm_s16le",
            "-y", // overwrite
            output_audio_path,
        ])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::other("ffmpeg failed"));
    }
    Ok(())
}
