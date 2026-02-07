// ==============================================
// IMPORTS
// ==============================================

use clap::{Parser, Subcommand};
use dirs::home_dir;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// New imports
use chrono::{DateTime, Local};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::process::Command;

// ==============================================
// CONSTANTS / CONFIG
// ==============================================

/// All default areas ICEland will manage.
/// Keeping them in one place avoids typos.
const DEFAULT_AREAS: &[&str] = &["work", "math", "learning", "gaming", "traveling", "trading"];

// ==============================================
// DATA STRUCTURES
// ==============================================

#[derive(Serialize, Deserialize, Debug)]
struct Session {
    area: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
}

// ==============================================
// CLI DEFINITION
// ==============================================

#[derive(Parser)]
#[command(
    name = "iceland",
    about = "Create and manage focused digital areas with tools, links, notes, and time tracking"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize ICEland and create all default areas
    Init,

    /// Switch to an area (writes it as current and shows its links)
    Switch {
        /// Name of the area, e.g. work, math, learning
        area: String,
    },

    /// TUI selection with arrow keys
    Tui,

    /// Show which area is currently active
    Status,

    /// Show time statistics per area
    Stats,

    /// Reset something inside an area (browser or notes)
    ///
    /// Example:
    ///   iceland destroy work browser
    ///   iceland destroy math notes
    Destroy {
        /// Area name
        area: String,
        /// What to reset: "browser" or "notes"
        target: String,
    },

    /// Add a note to an area's notes file
    ///
    /// Example:
    ///   iceland notes math "do exercise 5"
    Notes {
        /// Area name
        area: String,
        /// Text of the note
        text: String,
    },
}

// ==============================================
// MAIN
// ==============================================

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => init_iceland(),
        Commands::Switch { area } => switch_area(&area),
        Commands::Tui => tui_select_area(),
        Commands::Status => show_status(),
        Commands::Stats => show_stats(),
        Commands::Destroy { area, target } => destroy_in_area(&area, &target),
        Commands::Notes { area, text } => add_note(&area, &text),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
    }
}

// ==============================================
// PATH HELPERS
// ==============================================

/// Base folder: ~/.iceland (hidden folder)
fn get_iceland_dir() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".iceland");
    path
}

/// Current-area config file: ~/.iceland_current
fn get_config_file() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".iceland_current");
    path
}

/// Sessions log file: ~/.iceland/sessions.csv
fn get_sessions_file() -> PathBuf {
    let mut path = get_iceland_dir();
    path.push("sessions.csv");
    path
}

// ==============================================
// INIT COMMAND
// ==============================================

fn init_iceland() -> io::Result<()> {
    let base = get_iceland_dir();

    if !base.exists() {
        fs::create_dir_all(&base)?;
        println!("Created base folder: {}", base.display());
    } else {
        println!("Base folder already exists: {}", base.display());
    }

    for area in DEFAULT_AREAS {
        let mut area_path = base.clone();
        area_path.push(area);

        if !area_path.exists() {
            fs::create_dir_all(&area_path)?;
            println!("Created area: {} ({})", area, area_path.display());
        }

        // Ensure minimal common structure (most areas have at least notes)
        let notes_dir = area_path.join("notes");
        if !notes_dir.exists() {
            fs::create_dir_all(&notes_dir)?;
        }

        // Area-specific setup
        match *area {
            "math" => init_math_area(&area_path)?,
            "learning" => init_learning_area(&area_path)?,
            "work" => init_work_area(&area_path)?,
            "gaming" => init_gaming_area(&area_path)?,
            "traveling" => init_traveling_area(&area_path)?,
            "trading" => init_trading_area(&area_path)?,
            _ => {}
        }
    }

    // Default current area: learning (one of your real areas)
    let config = get_config_file();
    let mut file = File::create(config)?;
    writeln!(file, "learning")?;

    println!("ICEland initialized with: {}.", DEFAULT_AREAS.join(", "));
    Ok(())
}

// --- Area-specific helpers (keep init_iceland simpler) ---

fn init_math_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("flashcards"))?;
    fs::create_dir_all(area_path.join("browser_firefox"))?;

    let mut links = File::create(area_path.join("links.txt"))?;
    writeln!(links, "Math resources:")?;
    writeln!(links, "https://www.khanacademy.org")?;
    Ok(())
}

fn init_learning_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("summaries"))?;
    fs::create_dir_all(area_path.join("browser_comet"))?;

    let mut links = File::create(area_path.join("links.txt"))?;
    writeln!(links, "Primuss:")?;
    writeln!(links, "https://www3.primuss.de/")?;
    writeln!(links, "Wikipedia:")?;
    writeln!(links, "https://www.wikipedia.org")?;
    writeln!(links, "ChatGPT:")?;
    writeln!(links, "https://chat.openai.com")?;
    Ok(())
}

fn init_work_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("projects"))?;
    fs::create_dir_all(area_path.join("docs"))?;
    fs::create_dir_all(area_path.join("browser_profile"))?;
    Ok(())
}

fn init_gaming_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("games"))?;
    fs::create_dir_all(area_path.join("clips"))?;
    fs::create_dir_all(area_path.join("browser_profile"))?;

    let mut links = File::create(area_path.join("links.txt"))?;
    writeln!(links, "Easy NAND game:")?;
    writeln!(links, "https://nandgame.com")?;
    writeln!(links, "Simple Snake game:")?;
    writeln!(links, "https://playsnake.org")?;
    Ok(())
}

fn init_traveling_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("plans"))?;

    let mut links = File::create(area_path.join("links.txt"))?;
    writeln!(links, "News for traveling:")?;
    writeln!(links, "https://www.bbc.com/news")?;
    writeln!(links, "https://www.theguardian.com/world")?;
    Ok(())
}

fn init_trading_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("analysis"))?;

    let mut links = File::create(area_path.join("links.txt"))?;
    writeln!(links, "Forex Factory:")?;
    writeln!(links, "https://www.forexfactory.com/")?;
    Ok(())
}

// ==============================================
// TIME TRACKING HELPERS
// ==============================================

fn read_current_area() -> Option<String> {
    let config = get_config_file();
    if !config.exists() {
        return None;
    }
    let content = fs::read_to_string(config).ok()?;
    Some(content.trim().to_string())
}

fn record_session(old_area: &str) -> io::Result<()> {
    // For now we record an instant session from now to now.
    // Later you can store a real start time and reuse it.
    let now = Local::now();
    let session = Session {
        area: old_area.to_string(),
        start: now,
        end: now,
    };

    let sessions_file = get_sessions_file();
    let file_exists = sessions_file.exists();

    let mut file = File::options()
        .create(true)
        .append(true)
        .open(&sessions_file)?;

    if !file_exists {
        writeln!(file, "area,start,end")?;
    }

    writeln!(
        file,
        "{},{},{}",
        session.area,
        session.start.to_rfc3339(),
        session.end.to_rfc3339()
    )?;

    Ok(())
}

// Optional browser hook; keep simple and configurable later.
fn launch_browser_for_area(area: &str) -> io::Result<()> {
    // Example: firefox with profile equal to area name.
    // You can change command/args per OS or via config later.
    let _child = Command::new("firefox")
        .arg("-P")
        .arg(area)
        .spawn()?;

    println!("Launched browser for area: {area}");
    Ok(())
}

// ==============================================
// SWITCH COMMAND
// ==============================================

fn switch_area(area: &str) -> io::Result<()> {
    let base = get_iceland_dir();
    let mut target = base.clone();
    target.push(area);

    if !target.exists() {
        println!("Area '{}' does not exist. Run `iceland init` first.", area);
        return Ok(());
    }

    // Time tracking: record a short session for previous area
    if let Some(old_area) = read_current_area() {
        if old_area != area {
            if let Err(e) = record_session(&old_area) {
                eprintln!("Warning: could not record session for {old_area}: {e}");
            }
        }
    }

    // Write new area
    let config = get_config_file();
    let mut file = File::create(config)?;
    writeln!(file, "{}", area)?;

    println!("Switched to area: {}", area);
    println!("Your area folder: {}", target.display());

    // Show links
    let links_path = target.join("links.txt");
    if links_path.exists() {
        println!("\nUseful links for {}:", area);
        let links = fs::read_to_string(links_path)?;
        println!("{links}");
    }

    // Optional: launch browser with profile = area
    if let Err(e) = launch_browser_for_area(area) {
        eprintln!("Warning: could not launch browser for {area}: {e}");
    }

    Ok(())
}

// ==============================================
// STATUS COMMAND
// ==============================================

fn show_status() -> io::Result<()> {
    let config = get_config_file();

    if !config.exists() {
        println!("ICEland is not initialized. Run `iceland init` first.");
        return Ok(());
    }

    let current = fs::read_to_string(config).unwrap_or_else(|_| "unknown".to_string());
    let current_trimmed = current.trim();

    let mut area_path = get_iceland_dir();
    area_path.push(current_trimmed);

    println!("Current area: {}", current_trimmed);
    println!("Path: {}", area_path.display());

    Ok(())
}

// ==============================================
// STATS COMMAND
// ==============================================

fn show_stats() -> io::Result<()> {
    let sessions_file = get_sessions_file();
    if !sessions_file.exists() {
        println!("No sessions recorded yet.");
        return Ok(());
    }

    let content = fs::read_to_string(sessions_file)?;
    let mut totals: HashMap<String, i64> = HashMap::new();

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // header
        }
        let parts: Vec<_> = line.split(',').collect();
        if parts.len() != 3 {
            continue;
        }
        let area = parts[0].to_string();
        let start = DateTime::parse_from_rfc3339(parts[1])
            .ok()
            .map(|dt| dt.with_timezone(&Local));
        let end = DateTime::parse_from_rfc3339(parts[2])
            .ok()
            .map(|dt| dt.with_timezone(&Local));

        if let (Some(s), Some(e)) = (start, end) {
            let secs = (e - s).num_seconds();
            *totals.entry(area).or_insert(0) += secs;
        }
    }

    println!("Time per area (seconds, approximate):");
    for (area, secs) in totals {
        println!("- {area}: {secs}s");
    }

    Ok(())
}

// ==============================================
// DESTROY COMMAND (RESET PART OF AN AREA)
// ==============================================

fn destroy_in_area(area: &str, target: &str) -> io::Result<()> {
    let mut path = get_iceland_dir();
    path.push(area);

    if !path.exists() {
        println!("Area '{}' does not exist.", area);
        return Ok(());
    }

    match target {
        "browser" => {
            let firefox = path.join("browser_firefox");
            let comet = path.join("browser_comet");
            let generic = path.join("browser_profile");

            if firefox.exists() {
                fs::remove_dir_all(&firefox)?;
                fs::create_dir_all(&firefox)?;
                println!("Reset Firefox browser for {}", area);
            }

            if comet.exists() {
                fs::remove_dir_all(&comet)?;
                fs::create_dir_all(&comet)?;
                println!("Reset Comet browser for {}", area);
            }

            if generic.exists() {
                fs::remove_dir_all(&generic)?;
                fs::create_dir_all(&generic)?;
                println!("Reset browser profile for {}", area);
            }
        }

        "notes" => {
            let notes = path.join("notes");
            if notes.exists() {
                fs::remove_dir_all(&notes)?;
                fs::create_dir_all(&notes)?;
                println!("Destroyed and recreated notes for {}", area);
            } else {
                println!("No notes folder found for {}", area);
            }
        }

        _ => {
            println!("Unknown target. Use: browser or notes.");
        }
    }

    Ok(())
}

// ==============================================
// NOTES COMMAND
// ==============================================

fn add_note(area: &str, text: &str) -> io::Result<()> {
    let mut path = get_iceland_dir();
    path.push(area);
    path.push("notes");

    if !path.exists() {
        println!("Notes folder does not exist. Run `iceland init` first.");
        return Ok(());
    }

    let mut file = File::options()
        .create(true)
        .append(true)
        .open(path.join("my_notes.txt"))?;

    writeln!(file, "{}", text)?;
    println!("Added note to {}/notes/my_notes.txt", area);

    Ok(())
}

// ==============================================
// TUI COMMAND
// ==============================================

fn tui_select_area() -> io::Result<()> {
    let areas: Vec<String> = DEFAULT_AREAS.iter().map(|s| s.to_string()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an area")
        .items(&areas)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let chosen = &areas[selection];
    switch_area(chosen)
}
