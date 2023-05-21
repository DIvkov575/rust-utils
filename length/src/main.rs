use std::fs::{self};
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    // args
    let args: Vec<String> = std::env::args().collect();

    match args[1].as_str() {
        "file" => {
            let contents: String = fs::read_to_string(&args[2])?;
            let a = contents.len();
            print!("{}", a);
        }
        "lines" => {
            let lines_count = BufReader::new(fs::File::open(&args[2])?)
                .lines()
                .collect::<Vec<_>>()
                .len();
            // let stdout = io::stdout();
            // let mut handle = stdout.lock();
            print!("{}", lines_count);
        }
        "str" => {
            print!("{:#?}", args[2].len());
        }

        _ => {
            print!("Please specify (file, lines, or str) for first param, followed by corresponding value");
        }
    }
    Ok(())
}
