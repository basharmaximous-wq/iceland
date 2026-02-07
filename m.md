# ICEland


**Your Focused Digital Areas on One Device**


---


## Overview


ICEland is a Rust-based CLI tool to separate your digital life into focused areas:


- `work`, `learning`, `math`, `gaming`, `traveling`, `trading`
- Each area has its own **folders, notes, links, and browser profile**
- Switch areas quickly via **CLI** or **TUI**
- Logs basic **time spent per area**


---


## Installation


### From Source


```bash
git clone[ https://github.com/YOUR-USERNAME/iceland.git](https://github.com/basharmaximous-wq/iceland.git) 
cd iceland
cargo build --release
# Copy binary to PATH
cp target/release/iceland ~/bin/
Using Prebuilt Binaries

Linux x86_64: iceland-linux-x86_64

Windows: iceland.exe

Download from the GitHub Actions artifacts.

Testing ICEland

Initialize areas:

iceland init

Output example:

Created base folder: C:\Users\basha\.iceland
Created area: work (...)
Created area: math (...)
...
ICEland initialized with: work, math, learning, gaming, traveling, trading.

Check current area:

iceland status

Switch areas (CLI):

iceland switch learning
iceland switch gaming

Switch areas (TUI):

iceland tui

Use arrow keys and press Enter to select an area.

Add notes:

iceland notes math "Review integrals"

Reset parts of an area:

iceland destroy gaming browser
iceland destroy math notes

View time stats:

iceland stats
Customization

Browser command: change launch_browser_for_area in main.rs to match your preferred browser or OS.

Links: edit ~/.iceland/<area>/links.txt to add your own favorite resources.

Folder structure: modify the init_<area>_area functions to add custom folders.

TUI colors: customize ColorfulTheme in tui_select_area.

Important Paths

Base directory: ~/.iceland/

Current area file: ~/.iceland_current

Notes per area: ~/.iceland/<area>/notes/my_notes.txt

Links per area: ~/.iceland/<area>/links.txt

Sessions log: ~/.iceland/sessions.csv

Future Ideas

More accurate time tracking (start/stop sessions)

Cloud backup of notes

Integration with task managers or calendars

Desktop GUI version
