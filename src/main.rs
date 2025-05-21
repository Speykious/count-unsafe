#![forbid(unsafe_code)]

mod count;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to count in.
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let counters = count::find_unsafe_recursively(&args.path)?;

    println!("{:<12} | {:^6} | {:^6}", "", "Safe", "Unsafe");
    println!("{:-<12} | {:-^6} | {:-^6}", "", "", "");

    for (name, counter) in [
        ("Functions", counters.functions),
        ("Expressions", counters.exprs),
        ("Item impls", counters.item_impls),
        ("Item traits", counters.item_traits),
        ("Methods", counters.methods),
    ] {
        println!("{:<12} | {:^6} | {:^6}", name, counter.safe, counter.unsafe_);
    }

    Ok(())
}
