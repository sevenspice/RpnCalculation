mod rpn; // このクレート(プロジェクト)内で使用するモジュールの宣言

use std::path::PathBuf;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use clap::{Parser, Subcommand};
use anyhow::Result;
use rpn::Calculator;

#[derive(Parser)]
#[clap(
    author  = "@BananaPepperTK", 
    version = "1.0.0", 
    name    = "RpnCalclator",
    about   = "Super awesome rpn calclator"
)]
struct Cli {
    #[clap(short, long, parse(from_os_str), value_name="FILE")]
    file: Option<PathBuf>,
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
    #[clap(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Console {}
}

fn run<R: BufRead>(reader: R, verbose: usize) -> Result<()> {
    let calc = Calculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{:#?}", error)
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 標準入力から直接入力
    match &cli.command {
        Some(_) => {
            let stdin = stdin();
            let reader = stdin.lock();
            let _ = run(reader, cli.verbose);
        },
        None => {}
    }

    // ファイルから入力
    if let Some(file_path) = cli.file.as_deref() {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let _ = run(reader, cli.verbose);
    }

    Ok(())
}
