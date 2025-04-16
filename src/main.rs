use clap::Parser;
use cli_toolbox::cli::{Menu, System};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// scent: the Ninth Sentinel’s strongest sense
#[derive(Parser)]
struct Args {
    /// Path to config file
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct Command {
    name: String,
    shorthand: Option<String>,
    run: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    commands: Vec<Command>,
}


fn main() {
    let mut sys = System::new("scent".to_string());
    
    let args = Args::parse();
    
    let config_path = find_config_file(&args)
    .expect("No config file found! Please create ~/.config/scent/scent.yaml or use --config.");

    println!("Using config: {}", config_path.display());
    
    let file_content = fs::read_to_string(&config_path)
    .expect("Failed to read config file");

    let mut config: Config = serde_yaml::from_str(&file_content)
    .expect("Failed to parse YAML config");

    for cmd in &mut config.commands {
        if let Some(sh) = &cmd.shorthand {
            if sh == "sc" {
                println!("Warning: 'sc' is a reserved shorthand for opening the scent config file and won't be accessable.");
                cmd.shorthand = None;
            }
        }
    }
        
    println!("Loaded commands:");
    for cmd in &config.commands {
        sys.add_program(cmd.name.clone(), my_fn);
        println!(
            "- {} (shorthand: {}) => {}",
            cmd.name,
            cmd.shorthand.as_deref().unwrap_or("none"),
            cmd.run
        );
    }

    sys.menu();
    
    // Now you can use `config.commands` in your menu logic!
}

fn my_fn() {
    println!("ello");
}

fn find_config_file(args: &Args) -> Option<PathBuf> {
    // 1. Command-line argument
    if let Some(ref path) = args.config {
        if path.exists() {
            return Some(path.clone());
        } else {
            eprintln!("Config file not found: {}", path.display());
            std::process::exit(1);
        }
    }

    // 2. User config: ~/.config/scent/scent.yaml
    if let Some(mut path) = dirs::config_dir() {
        path.push("scent/scent.yaml");
        if path.exists() {
            return Some(path);
        }
    }

    // 3. System config: /etc/scent/scent.yaml
    let sys_path = PathBuf::from("/etc/scent/scent.yaml");
    if sys_path.exists() {
        return Some(sys_path);
    }

    // 4. Local file: ./scent.yaml (for dev/testing)
    let local = PathBuf::from("scent.yaml");
    if local.exists() {
        return Some(local);
    }

    None
}