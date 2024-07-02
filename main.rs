use clap::{Arg, Command};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use pretty_bytes::converter::convert;
use std::time::Instant;
use humantime::format_duration;

fn main() {
    let matches = Command::new("nettool")
        .version("1.0")
        .author("Nathan Oyewole <nathanoyewole7@gmail.com>")
        .about("Analyzes disk usage of directories")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Sets the path to analyze")
                .default_value("."),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let start = Instant::now();
    let total_size = analyze_disk_usage(path);
    let duration = start.elapsed();

    println!("Total size: {}", convert(total_size as f64));
    println!("Time taken: {}", format_duration(duration).to_string());
}

fn analyze_disk_usage<P: AsRef<Path>>(path: P) -> u64 {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} [{elapsed}] {msg}")
        .expect("Error setting progress bar style"));

    let mut total_size = 0;

    for entry in WalkDir::new(path) {
        let entry = entry.expect("Error reading directory entry");
        if entry.file_type().is_file() {
            let metadata = fs::metadata(entry.path()).expect("Unable to read metadata");
            total_size += metadata.len();
        }

        pb.set_message(format!("Processing: {}", entry.path().display()));
    }

    pb.finish_with_message("Analysis complete.");
    total_size
}

