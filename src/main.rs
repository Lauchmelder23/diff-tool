mod diff;
mod grid;

use clap::Parser;
use clap_derive::Parser;
use diff::Diff;


#[derive(Parser)]
struct Args {
    first: String,
    second: String
}

fn main() {
    let args = Args::parse();

    let diff = match Diff::new(args.first, args.second) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    let inputs = diff.get_input();
    println!("First: {}\nSecond: {}\n", inputs.0, inputs.1);

    println!("{diff}");
}
