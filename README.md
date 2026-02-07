# ICEland

**Your Focused Digital Areas on One Device**

---

## Overview

ICEland is a small **Rust-based command-line tool** that helps you separate your digital life into clear, role-based areas such as `work`, `learning`, `math`, `gaming`, `traveling`, and `trading`.

Instead of mixing everything into one chaotic home directory, ICEland creates structured environments so you can focus on one role at a time.

Each area gets:

- Its own folder under `~/.iceland/<area>/`
- A dedicated `notes/` folder
- Area-specific folders (e.g. `flashcards/`, `analysis/`, `plans/`)
- A `links.txt` file with useful, area-relevant links
- CLI commands to switch areas, reset parts of an area, and append notes
- A basic **time log** written to `~/.iceland/sessions.csv`
- Optional **browser launch** per area (simple `firefox -P <area>` by default)
- A simple **TUI selector** (`iceland tui`) to choose areas with arrow keys

ICEland is a **local tool**: it runs entirely on your computer and organizes your files. The website only explains and presents the idea.

---

## Why ICEland?

Many people use the same laptop for everything: university, self-study, gaming, and sometimes trading. The result is often messy:

- Study notes next to game screenshots  
- Trading links mixed with math videos in browser history  
- Constant context switching and loss of focus  

ICEland creates **separate “digital identities” (areas)** on one device without needing multiple users or virtual machines:

- In `learning` mode → Primuss, Wikipedia, and study notes  
- In `gaming` mode → games and clips, not exams and grades  
- In `trading` mode → analysis notes and Forex news, not Netflix  

With one simple CLI tool, you can prepare and switch between these worlds, and now also log time and pick areas via a TUI.

---

## Features & Commands

### 1. Initialize all areas

```bash
iceland init
This will:

Create a hidden base folder: ~/.iceland/

Create default areas:

work

math

learning

gaming

traveling

trading

Set up a notes/ folder in each area

Create area-specific structures such as:

math/: flashcards/, browser_firefox/, math links

learning/: summaries/, browser_comet/, links to Primuss, Wikipedia, ChatGPT

work/: projects/, docs/, browser_profile/

gaming/: games/, clips/, browser_profile/, game links

traveling/: plans/, travel/news links

trading/: analysis/, Forex links

Write the current area (default: learning) into ~/.iceland_current

2. Switch between areas (CLI)
bash
iceland switch learning
iceland switch gaming
iceland switch trading
This will:

Update ~/.iceland_current with the selected area

Print the path of that area

Display useful links from links.txt if it exists

Log a simple session entry for the previous area into ~/.iceland/sessions.csv

Example output:

text
Switched to area: learning
Your area folder: /home/user/.iceland/learning

Useful links for learning:
Primuss:
https://www3.primuss.de/
Wikipedia:
https://www.wikipedia.org
ChatGPT:
https://chat.openai.com
After switching, ICEland also tries to launch a browser using the area name as a profile, for example:

bash
firefox -P learning
You can adjust this behaviour in the source code to match your browser and operating system.

3. Switch between areas (TUI with arrow keys)
bash
iceland tui
This will:

Show a simple terminal UI listing all default areas

Let you move with up/down arrows and select with Enter

Internally call switch for the chosen area (including time logging and link display)

This is a quick way to jump between modes without typing the area name each time.

4. Check current status
bash
iceland status
Shows:

The currently active area

The filesystem path to that area

Example:

text
Current area: learning
Path: /home/user/.iceland/learning
5. Reset parts of an area (soft “destroy”)
Sometimes you want a fresh start in one part of an area without deleting everything.

bash
iceland destroy work browser
iceland destroy math notes
destroy <area> browser
Deletes and recreates browser-related folders in that area:

browser_firefox/

browser_comet/

browser_profile/

Useful if you want to clear local browser profile data for that area.

destroy <area> notes
Deletes and recreates the notes/ folder in that area.

This gives you a clean notes space without touching other files.

ICEland never deletes the entire ~/.iceland folder or an area itself in a single command to avoid accidents.

6. Append notes to an area
bash
iceland notes learning "Study Rust at 17:00"
iceland notes trading "Check economic calendar before entry"
This will:

Ensure the notes/ folder exists

Append your text as a new line to notes/my_notes.txt

Let you build a simple log or TODO list per area

7. Time tracking & stats (basic)
ICEland records basic session entries whenever you switch away from an area.

Session log file: ~/.iceland/sessions.csv

Format (CSV with header):

text
area,start,end
learning,2025-01-01T10:00:00+01:00,2025-01-01T10:05:00+01:00
gaming,2025-01-01T10:05:00+01:00,2025-01-01T10:30:00+01:00
You can get a simple summary with:

bash
iceland stats
This will:

Read sessions.csv

Parse all entries

Print total time per area (in seconds)

Example output:

text
Time per area (seconds, approximate):
- learning: 300s
- gaming: 1500s
Right now, sessions are recorded in a very simple way and are more a starting point for future, more accurate tracking (with real start/stop and daily reports).

Installation
Prerequisites
Rust toolchain (install via rustup)

cargo available in your terminal

Build from source
bash
git clone https://github.com/<your-username>/iceland.git
cd iceland
cargo build --release
The compiled binary will be at:

text
target/release/iceland
You can copy it to any directory in your PATH, for example:

bash
cp target/release/iceland ~/bin/
Prebuilt binaries
If you use the provided GitHub Actions workflow, you can download a statically linked Linux binary (e.g. *-unknown-linux-musl) from the Actions artifacts and run it directly on a compatible Linux system.

Usage Summary
bash
# First-time setup
iceland init

# Check current area
iceland status

# Switch areas (CLI)
iceland switch learning
iceland switch gaming
iceland switch trading

# Switch areas (TUI with arrow keys)
iceland tui

# Add notes to an area
iceland notes learning "finish assignment 3"
iceland notes math "review integrals"

# Reset browser data or notes for an area
iceland destroy gaming browser
iceland destroy math notes

# Show basic time stats per area
iceland stats
Important Paths
Base directory: ~/.iceland/

Current area file: ~/.iceland_current

Notes: ~/.iceland/<area>/notes/my_notes.txt

Links: ~/.iceland/<area>/links.txt

Sessions: ~/.iceland/sessions.csv

Project Structure
Core files
Cargo.toml — Rust package configuration and dependencies (clap, dirs, chrono, serde, dialoguer)

src/main.rs — CLI logic, filesystem operations, time logging, and TUI selector

website/index.html or docs/index.html — Marketing page for ICEland

Main concepts in main.rs
Uses clap for argument parsing and subcommands (init, switch, tui, status, stats, destroy, notes)

Uses dirs::home_dir() to locate ~

Uses standard fs and File APIs to create, modify, and reset folders

Stores current area in ~/.iceland_current

Logs simple sessions to ~/.iceland/sessions.csv

Uses dialoguer to implement a minimal TUI area selector

Personal Learning Goals
ICEland was built to practice:

Rust basics in a real project

CLI design and error handling with Result

Filesystem I/O in Rust

Using Linux-style folder structures for productivity

Structuring work, study, and free time into separate digital “identities”

Presenting a small tool with a website and a future “Pro” concept

Experimenting with time tracking and TUI libraries in Rust

Future Ideas
Possible next features:

More accurate time tracking (real start/stop sessions, idle detection, daily/weekly reports)

Configurable browser commands per area (Chrome profiles, Edge, different profiles per OS)

Integration with task managers or calendars (Todoist, Notion, Google Calendar)

Richer TUI dashboard (live timers, stats, area details) or a small desktop GUI

Cloud backup of notes and configuration

License
This project is currently used for a university exam and personal learning.
No explicit license is granted for production use.

text
undefined
