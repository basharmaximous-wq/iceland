#  **ICEland — Your Focused Digital Areas**

**From chaos to clarity — one command at a time.**

---
## Why I created ICEland

ICEland was born the night I realized my laptop had become a single, noisy room for everything.

University, self-study, gaming, trading, random ideas – all dumped into the same folders, the same browser, the same brain space. I’d open my laptop to learn, and five minutes later I was in a game, a series, or some chart I didn’t plan to check.

I didn’t want more apps, more accounts, or virtual machines. I wanted **clear islands of focus** on one machine.

So I built ICEland: a way to split my digital life into sharp, separate areas – and to have one place to write down every problem that bothered me and how I finally killed it.
---
## 🌍 Overview

ICEland is a **Rust-powered command-line assistant** that helps you separate your digital life into clear, purposeful “areas” such as:

`work • learning • math • gaming • traveling • trading`

Instead of letting everything live in one messy home directory, ICEland creates structured environments so you can **focus on one role at a time**.

Each area gets its own dedicated space under:

```
~/.iceland/<area>/
```

With:

* A personal **notes/** folder
* Area-specific subfolders (e.g., `flashcards/`, `analysis/`, `plans/`)
* A **links.txt** file containing useful, context-relevant websites
* Commands to switch, reset, and write notes
* A simple **time log** (`~/.iceland/sessions.csv`)
* Optional **browser launching per area**
* A lightweight **TUI selector** (`iceland tui`) using arrow keys

ICEland runs **entirely locally** on your computer — your data stays yours.

---

## 🎯 Why ICEland?

Most people use one laptop for everything:

* Studying
* Gaming
* Trading
* Traveling planning
* Personal projects

Over time, this leads to:

* Study notes next to game clips
* Trading charts mixed with math videos
* Constant distractions and broken focus

ICEland creates **separate digital identities** on a single device — no virtual machines, no extra users, no complexity.

### Example mindsets with ICEland:

* **Learning mode** → Primuss, Wikipedia, ChatGPT, study notes
* **Gaming mode** → Games and clips, zero school distractions
* **Trading mode** → Market analysis, Forex news, economic calendar
* **Travel mode** → Trip plans and news

One tool. Many worlds. Instant switching.

---

## ⚙️ Features & Commands

### 🚀 1. Initialize all areas

```bash
iceland init
```

This will:

* Create `~/.iceland/`
* Create default areas:

  * work
  * math
  * learning
  * gaming
  * traveling
  * trading
* Create a `notes/` folder in each area
* Build area-specific structures such as:

```
math/
  flashcards/
  browser_firefox/
  links.txt

learning/
  summaries/
  browser_comet/
  links.txt (Primuss, Wikipedia, ChatGPT)

work/
  projects/
  docs/
  browser_profile/

gaming/
  games/
  clips/
  browser_profile/
  links.txt

traveling/
  plans/
  links.txt (news & travel sites)

trading/
  analysis/
  links.txt (ForexFactory, market resources)
```

* Set default current area to **learning** in `~/.iceland_current`

---

### 🔄 2. Switch between areas (CLI)

```bash
iceland switch learning
iceland switch gaming
iceland switch trading
```

What happens:

* Updates `~/.iceland_current`
* Prints your new area path
* Displays useful links from `links.txt`
* Logs your session time in `~/.iceland/sessions.csv`
* Tries to open a browser profile for that area (e.g.):

  ```bash
  firefox -P learning
  ```

Example output:

```
Switched to area: learning
Your area folder: /home/user/.iceland/learning

Useful links for learning:
Primuss: https://www3.primuss.de/
Wikipedia: https://www.wikipedia.org
ChatGPT: https://chat.openai.com
```

---

### 🎛️ 3. Switch areas with a TUI (keyboard menu)

```bash
iceland tui
```

* Navigate with ↑ ↓
* Press **Enter** to select
* Behind the scenes, this calls `iceland switch` automatically

Perfect for fast context switching.

---

### 📊 4. Check your current status

```bash
iceland status
```

Example:

```
Current area: learning
Path: /home/user/.iceland/learning
```

---

### 🧹 5. Reset parts of an area (“destroy”)

Sometimes you want a fresh start **without deleting everything**.

```bash
iceland destroy work browser
iceland destroy math notes
```

#### `destroy <area> browser`

Resets browser-related folders:

* browser_firefox/
* browser_comet/
* browser_profile/

Useful if you want a clean browsing environment.

#### `destroy <area> notes`

Deletes and recreates:

```
~/.iceland/<area>/notes/
```

Your area remains — only notes are cleared.

> ICEland never deletes the whole area or `~/.iceland/` to avoid accidents.

---

### 📝 6. Add notes to any area

```bash
iceland notes learning "Study Rust at 17:00"
iceland notes trading "Check economic calendar"
```

This appends your text to:

```
~/.iceland/<area>/notes/my_notes.txt
```

Think of it as your lightweight area journal.

---

### ⏱️ 7. Basic time tracking

Every time you switch areas, ICEland logs your session to:

```
~/.iceland/sessions.csv
```

Example format:

```
area,start,end
learning,2025-01-01T10:00:00+01:00,2025-01-01T10:05:00+01:00
gaming,2025-01-01T10:05:00+01:00,2025-01-01T10:30:00+01:00
```

Get a summary with:

```bash
iceland stats
```

Example output:

```
Time per area (seconds):
- learning: 300s
- gaming: 1500s
```

---

## 💻 Installation

### Prerequisites

* Rust installed (`rustup`)
* `cargo` available in your terminal

### Build from source

```bash
git clone https://github.com/<your-username>/iceland.git
cd iceland
cargo build --release
```

Binary will be at:

```
target/release/iceland
```
## Prebuilt binaries

ICEland is built automatically by GitHub Actions for multiple targets:

- **Linux x86_64** (static, `x86_64-unknown-linux-musl`)
- **Windows x86_64** (`x86_64-pc-windows-gnu`, `.exe`)

You can download the latest binaries from:

- GitHub **Releases** (recommended)  
- Or from the **Actions** page → select a successful workflow run → **Artifacts**

Example files:

- `iceland-x86_64-unknown-linux-musl` – Linux 64‑bit binary
- `iceland-x86_64-pc-windows-gnu.exe` – Windows 64‑bit executable

After download:

```bash
# Linux
chmod +x iceland-x86_64-unknown-linux-musl
./iceland-x86_64-unknown-linux-musl init

# Windows (PowerShell / CMD)
iceland-x86_64-pc-windows-gnu.exe init

You can add it to your PATH:

```bash
cp target/release/iceland ~/bin/
```

---

## 📌 Important Paths

| Purpose        | Path                                   |
| -------------- | -------------------------------------- |
| Base directory | `~/.iceland/`                          |
| Current area   | `~/.iceland_current`                   |
| Notes          | `~/.iceland/<area>/notes/my_notes.txt` |
| Links          | `~/.iceland/<area>/links.txt`          |
| Time log       | `~/.iceland/sessions.csv`              |

---

## 🧠 Learning Goals Behind ICEland

ICEland was built to practice:

* Real-world Rust development
* CLI design with `clap`
* Filesystem automation
* Linux-style productivity tooling
* Time logging and TUI interfaces
* Personal developer workflow design

---

## 🚀 Future Ideas

Possible next features:

* More accurate time tracking
* Custom browser profiles per area
* Chrome/Edge support
* Dashboard-style TUI
* Daily/weekly reports
* Cloud backup for notes

---

## 📜 License

Built for a university exam and personal learning.
No production license granted yet.

---

**ICEland — Separate your worlds. Sharpen your focus.**
