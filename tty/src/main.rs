use std::{io, process};

use coreutils_core::os::tty::{Error::*, IsTTY, TTYName};

mod cli;

fn main() {
    let matches = cli::create_app().get_matches();

    let desc_stdin = io::stdin();

    let silent_flag = matches.is_present("silent");

    let res = TTYName::new(&desc_stdin);

    if !silent_flag {
        match res {
            Ok(tty) => println!("{}", tty),
            Err(err @ NotTTY) => eprintln!("tty: {}", err),
            Err(err) => {
                eprintln!("tty: {}", err);
                process::exit(1)
            },
        }
    }

    if !desc_stdin.is_tty() {
        process::exit(1);
    }
}
