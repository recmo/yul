#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

use cli_batteries::version;
use eyre::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::{fs::File, io::AsyncReadExt};
use yul_parser::parse;

#[derive(Clone, Debug, StructOpt)]
struct Options {
    /// Input Yul source file
    file: PathBuf,
}

async fn app(options: Options) -> Result<()> {
    // Read source
    let mut file = File::open(options.file).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // Parse
    let parse = parse(&contents);

    // Output parse tree
    println!("{}", parse.debug_tree());
    Ok(())
}

fn main() {
    cli_batteries::run(version!(), app);
}
