use test_sdxl::generate::{run, Args};

use anyhow::{Error as E, Result};

use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    run(args)
}



fn testing() {
    
}