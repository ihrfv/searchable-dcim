use searchable_dcim::{Config, DcimIndexer};
use std::process;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "We are going to analyze videos in the following folder: {}",
        config.get_dcim_path()
    );

    if let Err(err) = DcimIndexer::new(&config).index_videos() {
        eprintln!("Error during indexing files: {err}");
        process::exit(0);
    }
}
