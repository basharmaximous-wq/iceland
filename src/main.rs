// ==============================================
// IMPORTS
// ==============================================

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// ==============================================
// CONSTANTS & CONFIG
// ==============================================

const APP_NAME: &str = "iceland";
const CONFIG_FILE: &str = "config.toml";
const CURRENT_AREA_FILE: &str = "current_area";
const SESSIONS_FILE: &str = "sessions.csv";
const SESSION_START_FILE: &str = "session_start";

// Default configuration (used when no config exists)
const DEFAULT_AREAS: &[&str] = &["work", "math", "learning", "gaming", "traveling", "trading"];

// ==============================================
// DATA STRUCTURES
// ==============================================

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    areas: Vec<String>,
    browser_command: String, // e.g., "firefox -P {area}"
}

impl Default for Config {
    fn default() -> Self {
        Self {
            areas: DEFAULT_AREAS.iter().map(|s| s.to_string()).collect(),
            browser_command: "firefox -P {area}".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    area: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
}

#[derive(Debug, Clone, ValueEnum)]
enum DestroyTarget {
    Browser,
    Notes,
}

// ==============================================
// CLI DEFINITION
// ==============================================

#[derive(Parser)]
#[command(name = APP_NAME, version, about = "Manage focused digital areas with tools, links, notes, and time tracking", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// Initialize Iceland with default areas
    Init,

    /// List all available areas
    List,

    /// Switch to a specific area
    Switch { area: String },

    /// Interactive area selector (TUI)
    Tui,

    /// Show current area status
    Status,

    /// Show time statistics per area
    Stats,

    /// Manually start a session timer in the current area
    Start,

    /// Stop the current session and record it
    Stop,

    /// Destroy/reset a component in an area
    Destroy {
        area: String,
        #[arg(value_enum)]
        target: DestroyTarget,
    },

    /// Add a note to an area
    Notes { area: String, text: String },

    /// Study flashcards for an area
    Flashcards { area: String },

    /// Add a new custom area
    AddArea { name: String },

    /// Remove an existing area (deletes all its data)
    RemoveArea { name: String },

    /// Show session history for an area (or all areas)
    History {
        #[arg(short, long)]
        area: Option<String>,
    },
}

// ==============================================
// PATH HELPERS
// ==============================================

fn iceland_dir() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(format!(".{}", APP_NAME));
    path
}

fn config_path() -> PathBuf {
    iceland_dir().join(CONFIG_FILE)
}

fn current_area_file() -> PathBuf {
    iceland_dir().join(CURRENT_AREA_FILE)
}

fn sessions_file() -> PathBuf {
    iceland_dir().join(SESSIONS_FILE)
}

fn session_start_file() -> PathBuf {
    iceland_dir().join(SESSION_START_FILE)
}

fn area_dir(area: &str) -> PathBuf {
    iceland_dir().join(area)
}

// ==============================================
// CONFIG MANAGEMENT
// ==============================================

fn load_config() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

fn save_config(config: &Config) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    fs::write(config_path(), content)?;
    Ok(())
}

// ==============================================
// AREA HELPERS
// ==============================================

fn area_exists(area: &str) -> bool {
    area_dir(area).is_dir()
}

fn read_current_area() -> Result<Option<String>> {
    let path = current_area_file();
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path)?;
    Ok(Some(content.trim().to_string()))
}

fn write_current_area(area: &str) -> Result<()> {
    fs::write(current_area_file(), area)?;
    Ok(())
}

// ==============================================
// SESSION HANDLING
// ==============================================

fn record_session(area: &str, start: DateTime<Local>, end: DateTime<Local>) -> Result<()> {
    let file = sessions_file();
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!file.exists())
        .from_path(&file)?;

    let session = Session {
        area: area.to_string(),
        start,
        end,
    };
    wtr.serialize(session)?;
    wtr.flush()?;
    Ok(())
}

fn stop_current_session() -> Result<()> {
    let start_file = session_start_file();
    if !start_file.exists() {
        return Err(anyhow!("No active session. Use `start` first."));
    }

    let start_str = fs::read_to_string(&start_file)?;
    let start = DateTime::parse_from_rfc3339(&start_str)
        .context("Invalid start time format")?
        .with_timezone(&Local);

    let area = read_current_area()?.ok_or_else(|| anyhow!("No current area set"))?;
    let end = Local::now();

    record_session(&area, start, end)?;
    fs::remove_file(start_file)?;
    println!("Stopped session for '{}' ({} seconds)", area, (end - start).num_seconds());
    Ok(())
}

// ==============================================
// INIT
// ==============================================

fn init_iceland() -> Result<()> {
    let base = iceland_dir();
    fs::create_dir_all(&base)?;

    let config = Config::default();
    save_config(&config)?;

    for area in &config.areas {
        create_area_structure(area)?;
    }

    // Set initial current area (first in list)
    if let Some(first) = config.areas.first() {
        write_current_area(first)?;
    }

    println!("✅ Iceland initialized in {}", base.display());
    println!("Areas: {}", config.areas.join(", "));
    Ok(())
}

fn create_area_structure(area: &str) -> Result<()> {
    let area_path = area_dir(area);
    fs::create_dir_all(&area_path)?;

    // Subdirectories common to all areas
    fs::create_dir_all(area_path.join("notes"))?;
    fs::create_dir_all(area_path.join("flashcards"))?;

    // Area‑specific initialisation (can be extended)
    match area {
        "math" => {
            fs::create_dir_all(area_path.join("browser_firefox"))?;
            let mut links = File::create(area_path.join("links.txt"))?;
            writeln!(links, "Math resources:")?;
            writeln!(links, "https://www.khanacademy.org")?;
        }
        "learning" => {
            fs::create_dir_all(area_path.join("browser_comet"))?;
            let mut links = File::create(area_path.join("links.txt"))?;
            writeln!(links, "Primuss: https://www3.primuss.de/")?;
            writeln!(links, "Wikipedia: https://www.wikipedia.org")?;
            writeln!(links, "ChatGPT: https://chat.openai.com")?;
        }
        "work" => {
            fs::create_dir_all(area_path.join("projects"))?;
            fs::create_dir_all(area_path.join("docs"))?;
            fs::create_dir_all(area_path.join("browser_profile"))?;
        }
        "gaming" => {
            fs::create_dir_all(area_path.join("games"))?;
            fs::create_dir_all(area_path.join("clips"))?;
            fs::create_dir_all(area_path.join("browser_profile"))?;
        }
        "traveling" => {
            fs::create_dir_all(area_path.join("plans"))?;
        }
        "trading" => {
            fs::create_dir_all(area_path.join("analysis"))?;
        }
        _ => {
            // For custom areas, just create a basic structure
            let mut links = File::create(area_path.join("links.txt"))?;
            writeln!(links, "# Links for {}\n", area)?;
        }
    }

    Ok(())
}

// ==============================================
// SWITCH
// ==============================================

fn switch_area(new_area: &str) -> Result<()> {
    if !area_exists(new_area) {
        return Err(anyhow!("Area '{}' does not exist. Use `add-area` first.", new_area));
    }

    // Stop current session if any
    let start_file = session_start_file();
    if start_file.exists() {
        if let Err(e) = stop_current_session() {
            eprintln!("Warning: failed to stop previous session: {}", e);
        }
    }

    // Start new session
    fs::write(&start_file, Local::now().to_rfc3339())?;
    write_current_area(new_area)?;

    println!("🔄 Switched to area: {}", new_area);
    println!("   Path: {}", area_dir(new_area).display());

    // Show links if available
    let links_path = area_dir(new_area).join("links.txt");
    if links_path.exists() {
        println!("\n📌 Useful links:");
        let links = fs::read_to_string(links_path)?;
        println!("{}", links);
    }

    // Launch browser if configured
    let config = load_config()?;
    let browser_cmd = config.browser_command.replace("{area}", new_area);
    if !browser_cmd.is_empty() {
        let mut parts = browser_cmd.split_whitespace();
        if let Some(cmd) = parts.next() {
            let args: Vec<_> = parts.collect();
            if let Err(e) = Command::new(cmd).args(args).spawn() {
                eprintln!("Warning: could not launch browser: {}", e);
            } else {
                println!("🌐 Launched browser for area '{}'", new_area);
            }
        }
    }

    Ok(())
}

// ==============================================
// STATUS
// ==============================================

fn show_status() -> Result<()> {
    let current = read_current_area()?;
    match current {
        Some(area) => {
            println!("📍 Current area: {}", area);
            println!("   Path: {}", area_dir(&area).display());

            if session_start_file().exists() {
                println!("⏱️  Session timer is running.");
            } else {
                println!("⏸️  No active session.");
            }
        }
        None => {
            println!("❌ No current area set. Run `init` or `switch`.");
        }
    }
    Ok(())
}

// ==============================================
// STATS
// ==============================================

fn show_stats() -> Result<()> {
    let file = sessions_file();
    if !file.exists() {
        println!("No sessions recorded yet.");
        return Ok(());
    }

    let mut rdr = csv::Reader::from_path(file)?;
    let mut totals: HashMap<String, i64> = HashMap::new();

    for result in rdr.deserialize() {
        let session: Session = result?;
        let secs = (session.end - session.start).num_seconds();
        *totals.entry(session.area).or_insert(0) += secs;
    }

    println!("\n+------------+------------------+");
    println!("| Area       | Time (hh:mm)     |");
    println!("+------------+------------------+");

    for (area, secs) in totals {
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        println!("| {:<10} | {:>7}:{:02} h       |", area, hours, minutes);
    }

    println!("+------------+------------------+\n");
    Ok(())
}

// ==============================================
// SESSION COMMANDS
// ==============================================

fn start_session() -> Result<()> {
    let area = read_current_area()?.ok_or_else(|| anyhow!("No current area set"))?;
    let start_file = session_start_file();
    if start_file.exists() {
        return Err(anyhow!("Session already started. Use `stop` first."));
    }
    fs::write(&start_file, Local::now().to_rfc3339())?;
    println!("▶️  Timer started for area: {}", area);
    Ok(())
}

fn stop_session() -> Result<()> {
    stop_current_session()
}

// ==============================================
// DESTROY
// ==============================================

fn destroy_in_area(area: &str, target: DestroyTarget) -> Result<()> {
    let area_path = area_dir(area);
    if !area_path.exists() {
        return Err(anyhow!("Area '{}' does not exist.", area));
    }

    match target {
        DestroyTarget::Browser => {
            let browser_dirs = ["browser_firefox", "browser_comet", "browser_profile"];
            for name in &browser_dirs {
                let dir = area_path.join(name);
                if dir.exists() {
                    fs::remove_dir_all(&dir)?;
                    fs::create_dir_all(&dir)?;
                    println!("♻️  Reset browser directory: {}", name);
                }
            }
        }
        DestroyTarget::Notes => {
            let notes = area_path.join("notes");
            if notes.exists() {
                fs::remove_dir_all(&notes)?;
                fs::create_dir_all(&notes)?;
                println!("♻️  Reset notes for {}", area);
            } else {
                println!("No notes folder found for {}", area);
            }
        }
    }
    Ok(())
}

// ==============================================
// NOTES
// ==============================================

fn add_note(area: &str, text: &str) -> Result<()> {
    let notes_dir = area_dir(area).join("notes");
    if !notes_dir.exists() {
        return Err(anyhow!("Area '{}' does not exist or has no notes folder.", area));
    }

    let notes_file = notes_dir.join("my_notes.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&notes_file)?;

    writeln!(file, "{}", text)?;
    println!("📝 Note added to {}", notes_file.display());
    Ok(())
}

// ==============================================
// FLASHCARDS
// ==============================================

fn tui_flashcards(area: &str) -> Result<()> {
    let flashcards_dir = area_dir(area).join("flashcards");
    if !flashcards_dir.exists() {
        return Err(anyhow!("No flashcards found for area '{}'.", area));
    }

    let mut decks = vec![];
    for entry in fs::read_dir(&flashcards_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                decks.push(name.to_string());
            }
        }
    }

    if decks.is_empty() {
        println!("No flashcard decks available.");
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a deck")
        .items(&decks)
        .default(0)
        .interact()?;

    let deck_path = flashcards_dir.join(&decks[selection]);
    let file = File::open(deck_path)?;
    let reader = BufReader::new(file);

    let mut cards = vec![];
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if let Some((front, back)) = line.split_once('|') {
            cards.push((front.to_string(), back.to_string()));
        } else {
            eprintln!("Warning: line {} skipped (no '|' separator)", i + 1);
        }
    }

    if cards.is_empty() {
        println!("Deck is empty or malformed.");
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
        if i < cards.len() - 1 {
            println!("Press Enter for next card...");
            io::stdin().read_line(&mut input)?;
        }
    }
    println!("--- Finished deck ---");
    Ok(())
}

// ==============================================
// AREA MANAGEMENT
// ==============================================

fn list_areas() -> Result<()> {
    let config = load_config()?;
    let current = read_current_area()?.unwrap_or_default();

    println!("Available areas:");
    for area in &config.areas {
        let marker = if *area == current { "▶" } else { " " };
        println!("  {} {}", marker, area);
    }
    Ok(())
}

fn add_area(name: &str) -> Result<()> {
    let mut config = load_config()?;
    if config.areas.iter().any(|a| a == name) {
        return Err(anyhow!("Area '{}' already exists.", name));
    }

    create_area_structure(name)?;
    config.areas.push(name.to_string());
    save_config(&config)?;
    println!("✅ Area '{}' created.", name);
    Ok(())
}

fn remove_area(name: &str) -> Result<()> {
    let mut config = load_config()?;
    if !config.areas.iter().any(|a| a == name) {
        return Err(anyhow!("Area '{}' not found.", name));
    }

    // Confirm deletion
    println!("WARNING: This will delete all data for area '{}' (notes, flashcards, etc.).", name);
    print!("Type 'yes' to confirm: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() != "yes" {
        println!("Aborted.");
        return Ok(());
    }

    // Remove directory
    let area_path = area_dir(name);
    if area_path.exists() {
        fs::remove_dir_all(&area_path)?;
    }

    config.areas.retain(|a| a != name);
    save_config(&config)?;

    // If current area was removed, switch to first available or clear
    if let Some(current) = read_current_area()? {
        if current == name {
            if let Some(first) = config.areas.first() {
                write_current_area(first)?;
                println!("Switched to '{}'.", first);
            } else {
                fs::remove_file(current_area_file())?;
            }
        }
    }

    println!("🗑️  Area '{}' removed.", name);
    Ok(())
}

// ==============================================
// HISTORY
// ==============================================

fn show_history(area_filter: Option<String>) -> Result<()> {
    let file = sessions_file();
    if !file.exists() {
        println!("No sessions recorded.");
        return Ok(());
    }

    let mut rdr = csv::Reader::from_path(file)?;
    let mut sessions: Vec<Session> = rdr.deserialize().collect::<Result<_, _>>()?;

    // Filter by area if requested
    if let Some(area) = area_filter {
        sessions.retain(|s| s.area == area);
    }

    if sessions.is_empty() {
        println!("No sessions match the filter.");
        return Ok(());
    }

    println!("\n{:<12} | {:<25} | {:<25} | Duration", "Area", "Start", "End",);
    println!("{:-<12}-+-{:-<25}-+-{:-<25}-+-{:-<8}", "", "", "", "");

    for s in sessions {
        let start = s.start.format("%Y-%m-%d %H:%M:%S");
        let end = s.end.format("%Y-%m-%d %H:%M:%S");
        let duration = s.end - s.start;
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        println!(
            "{:<12} | {} | {} | {:2}:{:02}",
            s.area, start, end, hours, minutes
        );
    }
    println!();
    Ok(())
}

// ==============================================
// TUI SELECTOR
// ==============================================

fn tui_select_area() -> Result<()> {
    let config = load_config()?;
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an area")
        .items(&config.areas)
        .default(0)
        .interact()?;

    let chosen = &config.areas[selection];
    switch_area(chosen)
}

// ==============================================
// MAIN
// ==============================================

fn main() -> Result<()> {
    let cli = Cli::parse();
      match cli.command {
        Commands::Init => init_iceland(),
        Commands::List => list_areas(),
        Commands::Switch { area } => switch_area(&area),
        Commands::Tui => tui_select_area(),
        Commands::Status => show_status(),
        Commands::Stats => show_stats(),
        Commands::Start => start_session(),
        Commands::Stop => stop_session(),
        Commands::Destroy { area, target } => destroy_in_area(&area, target),
        Commands::Notes { area, text } => add_note(&area, &text),
        Commands::Flashcards { area } => tui_flashcards(&area),
        Commands::AddArea { name } => add_area(&name),
        Commands::RemoveArea { name } => remove_area(&name),
        Commands::History { area } => show_history(area),
    }
}
