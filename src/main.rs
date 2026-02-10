// ==============================================
// IMPORTS
// ==============================================

use clap::{Parser, Subcommand};
use dirs::home_dir;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::process::Command;

// ==============================================
// CONSTANTS
// ==============================================

const DEFAULT_AREAS: &[&str] =
    &["work", "math", "learning", "gaming", "traveling", "trading"];

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
    Init,
    Switch { area: String },
    Tui,
    Status,
    Stats,

    /// Manually start a session timer in the current area
    Start,

    /// Stop the current session and write it to sessions.csv
    Stop,

    Destroy { area: String, target: String },
    Notes { area: String, text: String },
    
    /// Study flashcards for a specific area
    Flashcards { area: String },
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
        Commands::Start => start_session(),
        Commands::Stop => stop_session(),
        Commands::Destroy { area, target } => destroy_in_area(&area, &target),
        Commands::Notes { area, text } => add_note(&area, &text),
        Commands::Flashcards { area } => tui_flashcards(&area),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
    }
}

// ==============================================
// PATH HELPERS
// ==============================================

fn get_iceland_dir() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".iceland");
    path
}

fn get_config_file() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".iceland_current");
    path
}

fn get_sessions_file() -> PathBuf {
    let mut path = get_iceland_dir();
    path.push("sessions.csv");
    path
}

/// Stores when the current session started
fn get_session_start_file() -> PathBuf {
    let mut path = get_iceland_dir();
    path.push("current_session_start");
    path
}

// ==============================================
// INIT
// ==============================================

fn init_iceland() -> io::Result<()> {
    let base = get_iceland_dir();

    if !base.exists() {
        fs::create_dir_all(&base)?;
        println!("Created base folder: {}", base.display());
    }

    for area in DEFAULT_AREAS {
        let mut area_path = base.clone();
        area_path.push(area);

        if !area_path.exists() {
            fs::create_dir_all(&area_path)?;
            println!("Created area: {} ({})", area, area_path.display());
        }

        let notes_dir = area_path.join("notes");
        if !notes_dir.exists() {
            fs::create_dir_all(&notes_dir)?;
        }

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

    let config = get_config_file();
    let mut file = File::create(config)?;
    writeln!(file, "learning")?;

    println!("ICEland initialized with: {}.", DEFAULT_AREAS.join(", "));
    Ok(())
}

// (Area init helpers — unchanged from your version)
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
    Ok(())
}

fn init_traveling_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("plans"))?;
    Ok(())
}

fn init_trading_area(area_path: &Path) -> io::Result<()> {
    fs::create_dir_all(area_path.join("analysis"))?;
    Ok(())
}

// ==============================================
// TIME TRACKING
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
    let start_file = get_session_start_file();
    let sessions_file = get_sessions_file();

    let start_str =
        fs::read_to_string(&start_file).unwrap_or_else(|_| Local::now().to_rfc3339());

    let start = DateTime::parse_from_rfc3339(&start_str)
        .ok()
        .map(|dt| dt.with_timezone(&Local))
        .unwrap_or_else(Local::now);

    let end = Local::now();

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
        old_area,
        start.to_rfc3339(),
        end.to_rfc3339()
    )?;

    Ok(())
}

fn start_session() -> io::Result<()> {
    let area = read_current_area().unwrap_or("unknown".to_string());
    let start_file = get_session_start_file();
    let mut f = File::create(start_file)?;
    writeln!(f, "{}", Local::now().to_rfc3339())?;
    println!("Started timer for area: {}", area);
    Ok(())
}

fn stop_session() -> io::Result<()> {
    if let Some(area) = read_current_area() {
        record_session(&area)?;
        println!("Stopped timer for area: {}", area);
    } else {
        println!("No active area.");
    }
    Ok(())
}

// ==============================================
// SWITCH
// ==============================================

fn switch_area(area: &str) -> io::Result<()> {
    let base = get_iceland_dir();
    let mut target = base.clone();
    target.push(area);

    if !target.exists() {
        println!("Area '{}' does not exist. Run `iceland init` first.", area);
        return Ok(());
    }

    if let Some(old_area) = read_current_area() {
        if old_area != area {
            let _ = record_session(&old_area);
        }
    }

    let start_file = get_session_start_file();
    let mut f = File::create(start_file)?;
    writeln!(f, "{}", Local::now().to_rfc3339())?;

    let config = get_config_file();
    let mut file = File::create(config)?;
    writeln!(file, "{}", area)?;

    println!("Switched to area: {}", area);
    println!("Your area folder: {}", target.display());

    let links_path = target.join("links.txt");
    if links_path.exists() {
        println!("\nUseful links for {}:", area);
        let links = fs::read_to_string(links_path)?;
        println!("{links}");
    }

    let _ = launch_browser_for_area(area);
    Ok(())
}

fn launch_browser_for_area(area: &str) -> io::Result<()> {
    let _child = Command::new("firefox")
        .arg("-P")
        .arg(area)
        .spawn()?;
    println!("Launched browser for area: {area}");
    Ok(())
}

// ==============================================
// STATUS
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
// PRETTIER STATS TABLE
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
            continue;
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

    println!("\n+------------+------------------+");
    println!("| Area       | Time (seconds)   |");
    println!("+------------+------------------+");

    for (area, secs) in totals {
        println!("| {:<10} | {:>16}s |", area, secs);
    }

    println!("+------------+------------------+\n");
    Ok(())
}

// ==============================================
// DESTROY
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
// NOTES
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
// TUI
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

// ==============================================
// FLASHCARDS TUI
// ==============================================

fn tui_flashcards(area: &str) -> io::Result<()> {
    let mut area_path = get_iceland_dir();
    area_path.push(area);
    area_path.push("flashcards");

    if !area_path.exists() {
        println!("No flashcards found for area '{}'.", area);
        return Ok(());
    }

    // List decks (files in flashcards folder)
    let mut decks = vec![];
    for entry in fs::read_dir(&area_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                decks.push(name.to_string());
            }
        }
    }

    if decks.is_empty() {
        println!("No flashcard decks available in '{}'.", area_path.display());
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a flashcard deck")
        .items(&decks)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let deck_file = area_path.join(&decks[selection]);
    let file = File::open(deck_file)?;
    let reader = io::BufReader::new(file);

    // Each line: front|back
    let mut cards = vec![];
    for line in reader.lines() {
        let line = line?;
        if let Some((front, back)) = line.split_once('|') {
            cards.push((front.to_string(), back.to_string()));
        }
    }

    if cards.is_empty() {
        println!("Deck is empty.");
        return Ok(());
    }

    println!("\n--- Starting flashcards ---");
    for (i, (front, back)) in cards.iter().enumerate() {
        println!("\nCard {} of {}", i + 1, cards.len());
        println!("Front: {}", front);
        println!("Press Enter to reveal back...");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        println!("Back: {}", back);
        println!("Press Enter for next card...");
        io::stdin().read_line(&mut input)?;
    }

    println!("--- Finished deck ---");
    Ok(())
}
