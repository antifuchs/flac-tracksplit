use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use flac_tracksplit::split_one_file;
use rayon::prelude::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Pathnames of .flac files (with embedded CUE sheets) to split into tracks.
    paths: Vec<PathBuf>,

    /// Output directory into which to sort resulting per-track FLAC files.
    /// Tracks will be named according to this template:
    ///
    /// OUTPUT_DIR/<Album Artist>/<Release year> - <Album name>/<Trackno>.<Track title>.flac
    #[arg(long, default_value = "./")]
    output_dir: PathBuf,
}

fn main() {
    // Setup logging:
    let indicatif_layer = tracing_indicatif::IndicatifLayer::new();
    let filter = EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();
    let writer = indicatif_layer.get_stderr_writer();
    let app_log_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .compact()
        .with_writer(writer);
    tracing_subscriber::registry()
        .with(filter)
        .with(app_log_layer)
        .with(indicatif_layer)
        .init();

    let args = Args::parse();
    let base_path = args.output_dir.as_path();
    args.paths
        .into_par_iter()
        .try_for_each(|path| {
            split_one_file(&path, base_path)
                .map(|_| ())
                .with_context(|| format!("When splitting {:?}", path))
        })
        .expect("Error splitting the given files");
}
