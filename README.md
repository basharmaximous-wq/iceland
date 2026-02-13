

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:E0FFFF,100:00BFFF&height=220&section=header&text=ICEland&fontSize=60&fontColor=ffffff&animation=fadeIn&fontAlignY=35&desc=Your%20Digital%20Focus%20Environment&descAlignY=60&descAlign=50" />
</p>

<p align="center">
  <img src="https://readme-typing-svg.herokuapp.com?color=00F5FF&size=22&center=true&vCenter=true&width=700&lines=From+chaos+to+clarity+—+one+command+at+a+time.;Separate+your+worlds.;Sharpen+your+focus.;Destroy+clutter.+Renew+workflow.+Stay+in+control." />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Built%20With-Rust-black?style=for-the-badge&logo=rust&logoColor=orange" />
  <img src="https://img.shields.io/badge/Privacy-100%25%20Local-00F5FF?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Interface-CLI-0066FF?style=for-the-badge" />
</p>

---

#  ICEland — Your Digital Focus Environment

ICEland is a **Rust-powered CLI tool** that creates isolated digital environments ("areas") on your machine — helping you maintain deep focus by separating work, learning, gaming, trading, and other contexts.

Each area gets:
- 📝 Dedicated notes  
- 🌐 Separate browser profiles  
- 🧠 Flashcard decks  
- ⏱️ Automatic time tracking  

All data stays **100% local**.  
No telemetry. No cloud. No tracking.

> Wipe the past. Lock your privacy. Dive into total focus.

---
![🌟Overview](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=Overview&fontColor=000000&fontSize=22)




ICEland helps you:

- Separate your digital worlds
- Avoid mental context switching
- Track time spent in each focus mode
- Organize notes and flashcards per environment
- Maintain total privacy

Your machine becomes structured, intentional, and distraction-free.

---

# 📦 Installation & Setup

## Prerequisites

Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

---

### Build ICEland

```bash
# 1. Clone the repository
git clone https://github.com/basharmaximous-wq/iceland.git
cd iceland

# 2. Build release binary
cargo build --release

# 3. Create bin directory (if needed)
mkdir -p ~/bin
#if you faced an error check the troubleshooting below 👇🏽 
# 4. Copy binary
cp target/release/iceland ~/bin/

# 5. Add to PATH
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# 6. Verify
iceland --version
```
## 🔧 Troubleshooting

Permission issues:

```bash
sudo chown -R $USER:$USER .
cargo build --release --target-dir ./target_local
```
---

## 🚀 Quick Start

Initialize ICEland:

```bash
iceland init
```

This creates:

```text
~/.iceland/
├── sessions.csv
├── work/
├── math/
├── learning/
├── gaming/
├── traveling/
└── trading/
```

Start the TUI:

```bash
iceland tui
```

Check status:

```bash
iceland status
```

---

## 🎮 TUI Navigation

After running:

```bash
iceland tui
```

You’ll see:

```text
Select an area › work   math   learning   gaming   traveling   trading
```

Navigation:

- ↑ / ↓ → Move  
- Enter → Select  
- Ctrl + C → Exit  

The TUI automatically:

- Switches to selected area  
- Shows useful links  
- Launches browser profile (if configured)  
- Starts time tracking  

---

## 🛠️ Command Reference

```bash
iceland --help
iceland switch --help
```

---

## 📝 Testing Your Installation

```bash
iceland init
iceland switch learning
iceland status
iceland notes learning "Test note from installation"
cat ~/.iceland/learning/notes/my_notes.txt
iceland --help
```

---
---

## 🌐 Suggested Links Per Area

Each ICEland area contains a `links.txt` file with curated resources
to match that specific focus environment.

These links are automatically shown when switching areas
and help you enter the right mindset instantly.

### 🎮 Gaming Area

Example links you might include:

- 🕹️ Addictive browser games  
- 🧩 Quick mini-games  
- 🍿 "Snack" games for short breaks  

The goal: controlled fun without drifting into chaos.

---

### 📈 Trading Area

Example curated resources:

- Forex news and economic calendar from **ForexFactory**  
- Market analysis tools  
- Currency strength dashboards  

This keeps you focused only on trading-related information.

---

### 📚 Learning Area

Example resources:

- 🎓 Your university portal (e.g., Primus)  
- 📖 Online course platforms  
- 📑 Study reference websites  
- 🧠 Research databases  

Everything in one isolated learning zone.

---

### 💼 Work Area

- Internal dashboards  
- Project management tools  
- Documentation systems  
- Company portals  

Zero distractions.

---

### ➗ Math Area

- Symbolic calculators  
- Formula references  
- Math problem solvers  
- Online textbooks  

---

### ✈️ Traveling Area

- Flight comparison tools  
- Maps  
- Booking websites  
- Travel planning tools  

---

### 🔒 Why This Matters

By separating suggested links per area, ICEland:

- Reduces mental context switching  
- Prevents unrelated browsing  
- Creates intentional digital environments  
- Reinforces discipline through structure  

Your browser becomes aligned with your purpose.

## 📚 Flashcards

Create a deck:

```bash
cd ~/.iceland/math/flashcards/
nano algebra_basics.txt
```

Use this format:

```text
What is 2+2?|4
Derivative of x²|2x
Integral of 2x|x² + C
```

Study:

```bash
iceland flashcards math
```

---

## ⏱️ Time Tracking

Sessions are saved in:

```text
~/.iceland/sessions.csv
```

View stats:

```bash
iceland stats
```

Manual control:

```bash
iceland start
iceland stop
```

---

## 🖥️ Browser Integration

Example:

```bash
firefox -P math &
```

---



Binary not found:

```bash
ls -la ~/bin/
chmod +x ~/bin/iceland
which iceland
echo $PATH
```

---

## 🏠 Data & Privacy

All ICEland data lives locally:

```text
~/.iceland/
```

No servers. No accounts. No tracking.

---

## 📄 License

Built for educational purposes and personal productivity.

---

**Separate your worlds. Sharpen your focus.**  
ICEland — From chaos to clarity.
