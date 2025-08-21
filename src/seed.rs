use std::fs;

use anyhow::{bail, Result};
use camino::Utf8PathBuf;
use clap::Args;

use rust_embed::RustEmbed;

use crate::Context;

// TODO check if it's compressed
#[derive(RustEmbed)]
#[folder = "./seed/"]
struct SeedFiles;

#[derive(Debug, Args)]
#[command()]
pub struct SeedArgs {
    /// Directory where example recipes will be created
    ///
    /// The seed command will create a collection of example Cooklang
    /// recipes in this directory. These examples demonstrate various
    /// Cooklang features including ingredients, cookware, timers, and
    /// metadata. If not specified, uses the current directory.
    ///
    /// The directory will be created if it doesn't exist.
    #[arg(value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    output: Option<Utf8PathBuf>,
}

pub fn run(ctx: &Context, args: SeedArgs) -> Result<()> {
    let path = args.output.as_ref().unwrap_or(&ctx.base_path);

    if path.is_file() {
        bail!("{} is not a directory", path);
    }

    fs::create_dir_all(path).expect("Couldn't create output path");

    for seed in SeedFiles::iter() {
        SeedFiles::get(seed.as_ref()).map(|content| {
            let file = path.join(seed.as_ref());
            let parent = file.parent().expect("Invalid path");

            fs::create_dir_all(parent).expect("Failed to create directory");

            fs::write(file, content.data.as_ref())
        });
    }

    Ok(())
}
