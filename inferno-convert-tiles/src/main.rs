use std::{fs, io::Read};

use clap::Parser;
use inferno_tiles::inferno::InfernoTile;
use tracing::{debug, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser)]
struct Args {
    /// Valhalla tile tarball to convert.
    #[clap(short, long)]
    input: String,
    /// Log level.
    #[clap(short, long, default_value_t = Level::INFO)]
    log_level: Level,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(args.log_level.clone())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut archive = tar::Archive::new(fs::File::open(&args.input)?);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = if let Some(path) = entry.path()?.to_str() {
            path.to_string()
        } else {
            warn!("Skipping unparseable path for entry: {:?}", entry.header());
            continue;
        };
        if path.ends_with("index.bin") {
            debug!("Skipping conversion of index.bin");
            continue;
        }
        info!(r#"Converting "{}"..."#, path);
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        match InfernoTile::from_valhalla(&bytes) {
            Ok(_) => {
                info!(r#"Successfully converted "{}"!"#, path);
            }
            Err(err) => {
                warn!(r#"Failed to convert "{}": {}"#, path, err);
            }
        }
    }

    Ok(())
}
