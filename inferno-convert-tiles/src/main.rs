use std::{fs, io::Read};

use clap::Parser;
use inferno_tiles::inferno::InfernoTile;

#[derive(Debug, Parser)]
struct Args {
    /// Valhalla tile tarball to convert.
    #[clap(short, long)]
    input: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut archive = tar::Archive::new(fs::File::open(&args.input)?);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = format!("{:?}", entry.path()?);
        if path.ends_with("index.bin") {
            continue;
        }
        println!(r#"Converting "{}"..."#, path);
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        match InfernoTile::from_valhalla(&bytes) {
            Ok(_) => {
                println!(r#"Successfully converted "{}"!"#, path);
            }
            Err(err) => {
                eprintln!(r#"Failed to convert "{}": {}"#, path, err);
            }
        }
    }

    Ok(())
}
