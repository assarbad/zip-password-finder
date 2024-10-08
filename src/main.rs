mod args;
mod charsets;
mod finder_errors;
mod password_finder;
mod password_gen;
mod password_reader;
mod password_worker;
mod zip_utils;

use crate::args::{get_args, Arguments};
use crate::finder_errors::FinderError;
use crate::password_finder::password_finder;
use crate::password_finder::Strategy::{GenPasswords, PasswordFile};

use crate::charsets::charset_from_choice;
use std::path::Path;

fn main() {
    let result = main_result();
    std::process::exit(match result {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    });
}

fn main_result() -> Result<(), FinderError> {
    // CLI args
    let Arguments {
        input_file,
        workers,
        charset_choice,
        min_password_len,
        max_password_len,
        file_number,
        password_dictionary,
    } = get_args()?;

    let strategy = if let Some(dict_path) = password_dictionary {
        let path = Path::new(&dict_path);
        PasswordFile(path.to_path_buf())
    } else {
        let charset = charset_from_choice(&charset_choice)?;
        GenPasswords {
            charset,
            min_password_len,
            max_password_len,
        }
    };

    // use physical cores by default to avoid issues with hyper-threading
    let workers = workers.unwrap_or_else(num_cpus::get_physical);
    let start_time = std::time::Instant::now();
    let password = password_finder(&input_file, workers, file_number, &strategy)?;
    let elapsed = start_time.elapsed();
    // display pretty time
    let elapsed = humantime::format_duration(elapsed);
    println!("Time elapsed: {elapsed}");
    match password {
        Some(password) => println!("Password found:{password}"),
        None => println!("Password not found"),
    };
    Ok(())
}
