use clap::Parser;
use std::error::Error;
use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use std::process;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, default_value = "false")]
   recursive: bool,
}

fn search_dir(path: &Path, recusive: bool, prefix: String) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        let candidates = fs::read_dir(path)?;
        let entries = candidates
            .filter_map(|item| match item.ok() {
                Some(dir_entry) => {
                    if recusive {
                        Some(dir_entry)
                    } else {
                        if dir_entry.path().is_dir() {
                            return None;
                        }
                        Some(dir_entry)
                    }
                }
                None => None,
            })
            .collect::<Vec<_>>();

        for (i, entry) in entries.iter().enumerate() {
            let entry_path = entry.path();

            let new_prefix = if i == entries.len() - 1 {
                println!("{}└── {}", prefix, output(&entry_path)?);
                prefix.clone() + "    "
            } else {
                println!("{}├── {}", prefix, output(&entry_path)?);
                prefix.clone() + "│   "
            };

            search_dir(entry_path.as_path(), recusive, new_prefix)?
        }
    }

    Ok(())
}

fn output(path: &Path) -> io::Result<String> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let symlink = match fs::read_link(path) {
        Ok(v) => v,
        Err(_err) => PathBuf::new(),
    };

    if symlink.to_str().unwrap().is_empty() {
        return Ok(filename.to_string())
    } else {
        return Ok(format!("{} -> {}", filename, symlink.to_str().unwrap()))
    };
}

fn run(recusive: bool) -> Result<(), Box<dyn Error>> {
    let current_dir_path = env::current_dir()?;
    search_dir(current_dir_path.as_path(), recusive, "".to_string())?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args.recursive) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
