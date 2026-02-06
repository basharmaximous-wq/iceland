

ICEland 

**Your Focused Digital Areas on One Device**

---

## Overview

ICEland is a small **Rust-based command-line tool** that helps you separate your digital life into clear, role-based areas such as `work`, `learning`, `math`, `gaming`, `traveling`, and `trading`.

Instead of mixing everything into one chaotic home directory, ICEland creates structured environments so you can focus on one role at a time.

Each area gets:

* Its own folder under `~/.iceland/<area>/`
* A dedicated `notes/` folder
* Area-specific folders (e.g. `flashcards/`, `analysis/`, `plans/`)
* A `links.txt` file with useful, area-relevant links
* CLI commands to switch areas, reset parts of an area, and append notes

ICEland is a **local tool**: it runs entirely on your computer and organizes your files. The website only explains and presents the idea.

---

## Why ICEland?

Many people (including me) use the same laptop for everything: university, self-study, gaming, and sometimes trading. The result is often messy:

* Study notes next to game screenshots
* Trading links mixed with math videos in browser history
* Constant context switching and loss of focus

ICEland creates **separate “digital identities” (areas)** on one device without needing multiple users or virtual machines:

* In `learning` mode → Primuss, Wikipedia, and study notes
* In `gaming` mode → games and clips, not exams and grades
* In `trading` mode → analysis notes and Forex news, not Netflix

With one simple CLI tool, you can quickly prepare and switch between these worlds.

---

## Features & Commands

### 1. Initialize all areas

```bash
iceland init
```

This will:

* Create a hidden base folder: `~/.iceland/`
* Create default areas:

  * `work`
  * `math`
  * `learning`
  * `gaming`
  * `traveling`
  * `trading`
* Set up a `notes/` folder in each area
* Create area-specific structures such as:

  * `math/`: `flashcards/`, `browser_firefox/`, math links
  * `learning/`: `summaries/`, `browser_comet/`, links to Primuss, Wikipedia, ChatGPT
  * `work/`: `projects/`, `docs/`, `browser_profile/`
  * `gaming/`: `games/`, `clips/`, `browser_profile/`, game links
  * `traveling/`: `plans/`, travel/news links
  * `trading/`: `analysis/`, Forex links
* Write the current area (default: `learning`) into `~/.iceland_current`

---

### 2. Switch between areas

```bash
iceland switch learning
iceland switch gaming
iceland switch trading
```

This will:

* Update `~/.iceland_current` with the selected area
* Print the path of that area
* Display useful links from `links.txt` if it exists

Example output:

```
Switched to area: learning
Your area folder: /home/user/.iceland/learning

Useful links for learning:
Primuss:
https://www3.primuss.de/
Wikipedia:
https://www.wikipedia.org
ChatGPT:
https://chat.openai.com
```

---

### 3. Check current status

```bash
iceland status
```

Shows:

* The currently active area
* The filesystem path to that area

Example:

```
Current area: learning
Path: /home/user/.iceland/learning
```

---

### 4. Reset parts of an area (soft “destroy”)

Sometimes you want a fresh start in one part of an area without deleting everything.

```bash
iceland destroy work browser
iceland destroy math notes
```

#### `destroy <area> browser`

Deletes and recreates browser-related folders in that area:

* `browser_firefox/`
* `browser_comet/`
* `browser_profile/`

Useful if you want to clear local browser profile data for that area.

#### `destroy <area> notes`

Deletes and recreates the `notes/` folder in that area.

This gives you a clean notes space without touching other files.

> ICEland never deletes the entire `~/.iceland` folder or an area itself in a single command to avoid accidents.

---

### 5. Append notes to an area

```bash
iceland notes learning "Study Rust at 17:00"
iceland notes trading "Check economic calendar before entry"
```

This will:

* Ensure the `notes/` folder exists
* Append your text as a new line to `notes/my_notes.txt`
* Let you build a simple log or TODO list per area

---

## Installation

### Prerequisites

* Rust toolchain (install via `rustup`)
* `cargo` available in your terminal

### Build from source

```bash
git clone https://github.com/<your-username>/iceland.git
cd iceland
cargo build --release
```

The compiled binary will be at:

```
target/release/iceland
```

You can copy it to any directory in your `PATH`, for example:

```bash
cp target/release/iceland ~/bin/
```

---

### Prebuilt binaries

If you use the provided GitHub Actions workflow, you can download a statically linked Linux binary (`*-unknown-linux-musl`) from the Actions artifacts and run it directly on a compatible Linux system.

---

## Usage Summary

```bash
# First-time setup
iceland init

# Check current area
iceland status

# Switch areas
iceland switch learning
iceland switch gaming
iceland switch trading

# Add notes to an area
iceland notes learning "finish assignment 3"
iceland notes math "review integrals"

# Reset browser data or notes for an area
iceland destroy gaming browser
iceland destroy math notes
```

### Important Paths

* **Base directory:** `~/.iceland/`
* **Current area file:** `~/.iceland_current`
* **Notes:** `~/.iceland/<area>/notes/my_notes.txt`
* **Links:** `~/.iceland/<area>/links.txt`

---

## Project Structure

### Core files

* `Cargo.toml` — Rust package configuration and dependencies (`clap`, `dirs`)
* `src/main.rs` — CLI logic and filesystem operations
* `website/index.html` or `docs/index.html` — Marketing page for ICEland

### Main concepts in `main.rs`

* Uses **clap** for argument parsing and subcommands (`init`, `switch`, `status`, `destroy`, `notes`)
* Uses `dirs::home_dir()` to locate `~`
* Uses standard `fs` and `File` APIs to create, modify, and reset folders
* Stores current area in `~/.iceland_current`

---

## Personal Learning Goals

ICEland was built to practice:

* Rust basics in a real project
* CLI design and error handling with `Result`
* Filesystem I/O in Rust
* Using Linux-style folder structures for productivity
* Structuring work, study, and free time into separate digital “identities”
* Presenting a small tool with a website and a future “Pro” concept

---

## Future Ideas

Possible next features:

* Automatically launch a browser with the correct profile for each area
* Time tracking per area (learning vs gaming vs work)
* Integration with task managers or calendars
* A simple TUI (terminal UI) to select areas with arrow keys

---

## License

This project is currently used for a university exam and personal learning.
No explicit license is granted for production use.
