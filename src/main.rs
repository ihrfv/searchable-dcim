use searchable_dcim::{Config, DcimIndexer};
use std::process;

fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::build().unwrap_or_else(|err| {
        tracing::error!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    tracing::info!(
        "We are going to analyze videos in the following folder: {}",
        config.get_dcim_path()
    );

    if let Err(err) = DcimIndexer::new(&config).index_videos() {
        tracing::error!("Error during indexing files: {err}");
        process::exit(0);
    }
}
