use clap::Parser;

/// A small, leightweight TiddlyWiki server that supports `PUT` (DAV) saves.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Target TiddlyWiki HTML file.
    #[clap(short, long, parse(from_os_str))]
    pub target: std::path::PathBuf,

    /// Directory to store backups.
    #[clap(short, long, parse(from_os_str))]
    pub backup_dir: std::path::PathBuf,

    /// Port to serve on.
    #[clap(short, long, default_value_t = 8000)]
    pub port: u32,
}
