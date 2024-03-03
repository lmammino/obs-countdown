use clap::Parser;
use format::format;
use std::{
    fs::{canonicalize, File},
    io::{stdout, Write},
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};
mod delta_parser;
mod format;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file where to store the countdown.
    /// If the file does not exist, it will be created.
    /// If no file is provided, it will create a file
    //  called "obs-countdown.txt" in the current folder.
    #[arg(short, long, default_value = "obs-countdown.txt")]
    file: String,

    /// Refresh rate in milliseconds
    #[arg(short, long, default_value_t = 500)]
    refresh_rate: u64,

    /// The format string to use to render the remaining time.
    /// The format string is a string that can contain the following placeholders:
    /// - %h: hours
    /// - %H: hours, zero-padded
    /// - %m: minutes
    /// - %M: minutes, zero-padded
    /// - %s: seconds
    /// - %S: seconds, zero-padded
    #[arg(long, default_value = "%H:%M:%S")]
    format: String,

    /// The message to display when the countdown is over
    #[arg(long, default_value = "00:00:00")]
    final_message: String,

    ///Countdown duration expression (e.g. "1h 30m 10s")
    countdown: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let countdown_duration_expr = args.countdown.join(" ");

    let duration = delta_parser::parse(&countdown_duration_expr);
    if duration.is_err() {
        eprintln!(
            "Invalid countdown duration '{}': {}",
            countdown_duration_expr,
            duration.unwrap_err()
        );
        exit(1);
    }

    let duration = duration.unwrap();
    let starting_time = Instant::now();
    let target_time = starting_time + duration;

    // parses the file path
    let file_path = canonicalize(&args.file);
    if file_path.is_err() {
        eprintln!(
            "Failed to resolve file path '{}': {}",
            &args.file,
            file_path.unwrap_err()
        );
        exit(1);
    }
    let file_path = file_path.unwrap();

    loop {
        let now = Instant::now();
        let remaining = target_time.saturating_duration_since(now);
        let finished = remaining.as_secs() == 0;

        // generates the current file content
        let message = if finished {
            args.final_message.clone()
        } else {
            format(remaining, &args.format)
        };

        // print to stdout
        print!("  ⏳  {} -> {}\r", message, &file_path.display());
        let _ = stdout().flush();

        // creates the file if it doesn't exist
        if !&file_path.exists() {
            let file = File::create(&file_path);
            if file.is_err() {
                eprintln!(
                    "Failed to create file '{}': {}",
                    &file_path.display(),
                    file.unwrap_err()
                );
                exit(1);
            }
        }

        // write to file
        let file = File::create(&file_path);
        if file.is_err() {
            eprintln!(
                "Failed to open file '{}': {}",
                &file_path.display(),
                file.unwrap_err()
            );
            exit(1);
        }
        let mut file = file.unwrap();

        let result = file.write_all(message.as_bytes());
        if result.is_err() {
            eprintln!(
                "Failed to write to file '{}': {}",
                &file_path.display(),
                result.unwrap_err()
            );
            exit(1);
        }

        if finished {
            break;
        }

        sleep(Duration::from_millis(args.refresh_rate));
    }

    println!();
}
