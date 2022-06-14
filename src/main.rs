use std::error::Error as StdError;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use intra::Ford;

const ERROR_STR: &str = "I have no idea what you are talking about";

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// File to write to. Defaults to stdout. Ignored if in interactive mode.
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,

    /// File to read from. Defaults to stdin.
    #[clap(value_parser)]
    path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();

    let mut ford = Ford::new();

    if let Some(path) = &args.path {
        let file = BufReader::new(File::open(path)?);

        let mut out_file: Box<dyn Write> = if let Some(output) = args.output {
            Box::new(File::create(output)?)
        } else {
            Box::new(io::stdout())
        };

        for line in file.lines() {
            let line = line?;
            let result = ford
                .query(&line)
                .unwrap_or_else(|_| Some(ERROR_STR.to_string()));

            if let Some(line) = result {
                writeln!(out_file, "{line}")?;
            }
        }

        Ok(())
    } else {
        let mut rl = Editor::<()>::new();
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    let result = ford
                        .query(&line)
                        .unwrap_or_else(|_| Some(ERROR_STR.to_string()));

                    if let Some(line) = result {
                        println!("{line}");
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }
}
