# ICEland — Your Digital Focus Environment
**From chaos to clarity — one command at a time. Separate your worlds. Sharpen your focus.**

## 🌟 Overview
ICEland is a **Rust-powered CLI tool** that creates isolated digital environments ("areas") on your machine—helping you maintain focus by separating work, learning, gaming, trading, and other contexts. Each area gets its own notes, browser profiles, flashcards, and time tracking—all while keeping your data completely local and private.  

Wipe the past, lock your privacy, and dive into total focus.

Destroy clutter, renew your workflow, and stay fully in control

## 📦 Installation & Setup
### Prerequisites
- **Rust toolchain** (install via `rustup` if you haven't already):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env  # Or restart your terminal
```
## Cargo (comes with Rust)
Building ICEland
# 1. Clone or download the source
git clone <https://github.com/basharmaximous-wq/iceland.git>

cd iceland

# 2. Build the release binary
cargo build --release

# 3. Create a bin directory if it doesn't exist
mkdir -p ~/bin  
# Or 
/usr/local/bin for system-wide installation

# 4. Copy the binary to your PATH
cp target/release/iceland ~/bin/

# 5. Make sure ~/bin is in your PATH
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc  
# or 
~/.zshrc
source ~/.bashrc

# 6. Verify installation
iceland --version
## 🚀 Quick Start (First-Time Setup)
Initialize ICEland (creates all area directories):
```
iceland init
```
This creates:
~/.iceland/ (your main data directory)
Six default areas: work, math, learning, gaming, traveling, trading
Each with structured folders and links.txt
---
Start with the TUI (recommended for beginners):
---
```
iceland tui
```
---
Check your status at any time:
---
```
iceland status
```
## 🎮 Getting Started with TUI (Text User Interface)
After running iceland init:
```
iceland tui
```
You'll see:
```
Select an area › work   math   learning   gaming   traveling   trading
```
Navigation:
↑/↓ arrow keys to move

Press Enter to select

Press Ctrl+C to exit

The TUI automatically:

Switches to your chosen area

Shows useful links from that area's links.txt

Launches a dedicated browser profile (if configured)

Starts time tracking for that session
---
## 🛠️ Command Reference
Get help anytime:
---
##📝 Testing Your Installation
```
# 1. Initialize
iceland init

# 2. Switch to learning area
iceland switch learning  # Should show: "Switched to area: learning" and display links

# 3. Check status
iceland status  # Shows: Current area: learning, Path: ~/.iceland/learning

# 4. Add a test note
iceland notes learning "Test note from installation"

# 5. View the note
cat ~/.iceland/learning/notes/my_notes.txt

# 6. Check available commands
iceland --help
```
---
🏠 Data Storage & Privacy
Your Digital Home
All ICEland data lives in your home directory, completely private
---
```
~/.iceland/           # Main directory
├── sessions.csv      # Time tracking data
├── work/             # Work area
├── math/             # Math area
├── learning/         # Learning area
├── gaming/           # Gaming area
├── traveling/        # Traveling area
└── trading/          # Trading area
```
Area Structure
Each area contains:
```
math/
├── notes/              # Your private notes (append-only)
│   └── my_notes.txt
├── flashcards/         # Flashcard decks (*.txt files)
├── browser_firefox/    # Firefox profile for math
└── links.txt           # Useful math websites
```
Notes are stored locally—they never leave your computer.
---
## 📚 Using Flashcards
Creating Flashcard Decks
```
cd ~/.iceland/math/flashcards/
nano algebra_basics.txt
```
Add cards in format front|back:
```
What is 2+2?|4
Derivative of x²|2x
Integral of 2x|x² + C
```
Save (Ctrl+O) and exit (Ctrl+X).
Studying Flashcards
```
iceland flashcards math
```
---
Select your deck
Press Enter to reveal answers
Press Enter again for next card
---
Viewing Decks
---
```
ls ~/.iceland/math/flashcards/   # See all decks
cat ~/.iceland/math/flashcards/algebra_basics.txt  # View a specific deck
```
## ⏱️ Time Tracking
ICEland tracks time when switching areas. Data is saved in ~/.iceland/sessions.csv.
View Stats
```
iceland stats
```
Example
```
+------------+------------------+
| Area       | Time (seconds)   |
+------------+------------------+
| learning   |           7200s  |
| work       |           5400s  |
| math       |           3600s  |
+------------+------------------+
```
Manual Session Control
```
iceland start  # Start timer
iceland stop   # Stop & log session
```
## 🖥️ Browser Integration
```
# Switch to 'math' area
firefox -P math &
```
## 🔧 Troubleshooting
Permission Denied After Build
```
sudo chown -R $USER:$USER .
cargo build --release --target-dir ./target_local
```
Binary Not Found After Copy
```
ls -la ~/bin/
chmod +x ~/bin/iceland
which iceland
echo $PATH
```
## 📄 License
Built for educational purposes and personal productivity.
All data stays on your machine—no tracking, no telemetry.
## 🆘 Need Help?
```
iceland --help
iceland switch --help
```
---
Separate your worlds. Sharpen your focus. 
ICEland: From chaos to clarity — one command at a time.
---
