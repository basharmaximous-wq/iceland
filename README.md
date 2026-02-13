

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:E0FFFF,100:00BFFF&height=220&section=header&text=ICEland&fontSize=60&fontColor=ffffff&animation=fadeIn&fontAlignY=35&desc=Your%20Digital%20Focus%20Environment&descAlignY=60&descAlign=50" />
</p>

<p align="center">
  <img src="https://readme-typing-svg.herokuapp.com?color=00F5FF&size=22&center=true&vCenter=true&width=700&lines=From+chaos+to+clarity+—+one+command+at+a+time.;Separate+your+worlds.;Sharpen+your+focus.;Destroy+clutter.+Renew+workflow.+Stay+in+control." />
</p>

<p align="center">
## License

<div align="center">
  
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge&color=0a3d62&labelColor=0a3d62&logo=opensourceinitiative&logoColor=white)](https://choosealicense.com/licenses/mit/)
  
<p style="padding: 12px 30px; background: linear-gradient(135deg, #0a3d62, #0ea5e9); border-radius: 50px; box-shadow: 0 0 20px #0a3d62, 0 0 40px #0ea5e9; color: white; font-weight: bold; font-size: 1.2em; display: inline-block; border: 1px solid rgba(255,255,255,0.3);">
  ✨ MIT LICENSE ✨
</p>

</div>

<br>

## Built With

<div align="center">
  
<p style="padding: 12px 40px; background: linear-gradient(135deg, #8b5cf6, #0ea5e9); border-radius: 50px; box-shadow: 0 0 20px #8b5cf6, 0 0 40px #0ea5e9; color: white; font-weight: bold; font-size: 1.2em; display: inline-block; border: 1px solid rgba(255,255,255,0.3);">
  🦀 RUST 🦀
</p>

<br>
<br>

| Crate | Description |
|-------|-------------|
| [![clap](https://img.shields.io/badge/clap-⚡-blue?style=flat-square&color=0ea5e9)](https://github.com/clap-rs/clap) | CLI argument parsing |
| [![serde](https://img.shields.io/badge/serde-⚡-blue?style=flat-square&color=8b5cf6)](https://serde.rs/) | Serialization |
| [![chrono](https://img.shields.io/badge/chrono-⚡-blue?style=flat-square&color=0a3d62)](https://github.com/chronotope/chrono) | Date/time handling |
| [![dialoguer](https://img.shields.io/badge/dialoguer-⚡-blue?style=flat-square&color=10b981)](https://github.com/mitsuhiko/dialoguer) | Interactive TUI |

</div>
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
![🌟Overview](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=🌟Overview&fontColor=000000&fontSize=22)




ICEland helps you:

- Separate your digital worlds
- Avoid mental context switching
- Track time spent in each focus mode
- Organize notes and flashcards per environment
- Maintain total privacy

Your machine becomes structured, intentional, and distraction-free.

---
![📦 Installation & Setup](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=📦Installation&Setup&fontColor=000000&fontSize=22)



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
#6 make sure
iceland --version
```
![🔧 Troubleshooting](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=🔧Troubleshooting&fontColor=000000&fontSize=22)

---



Binary not found:

```bash
ls -la ~/bin/
chmod +x ~/bin/iceland
which iceland
echo $PATH
```

---
Permission issues:

```bash
sudo chown -R $USER:$USER .
cargo build --release --target-dir ./target_local
```
---
![🚀 Quick Start](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=🚀Quick_Start&fontColor=000000&fontSize=22)


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


Check status:

```bash
iceland status
```

---
![🎮 TUI Navigation](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=🎮TUInavigation&fontColor=000000&fontSize=22)



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


iceland list – shows all areas (current is marked with ▶).

iceland add-area <name> – creates a brand new area with its own notes, flashcards, and links.

iceland remove-area <name> – deletes an area and all its data (with confirmation
---
![🛠️ Command Reference](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=🛠️Command_Reference&fontColor=000000&fontSize=22)


| Command | Description |
|---------|-------------|
| `iceland init` | Create default areas + config |
| `iceland list` | Show areas (▶ marks current) |
| `iceland tui` | Interactive area selector |
| `iceland switch <area>` | Switch area + start session + show links |
| `iceland status` | Current area + session state |
| `iceland stats` | Time spent per area (table) |
| `iceland start/stop` | Manual session timer |
| `iceland notes <area> "text"` | Append note |
| `iceland flashcards <area>` | Study flashcards |
| `iceland destroy <area> browser` | Reset browser profile |
| `iceland destroy <area> notes` | Clear notes |
| `iceland add-area <name>` | New custom area |
| `iceland remove-area <name>` | Delete area + data |
| `iceland history [area]` | Session log |

**Full help:** `iceland --help`

## ✨ Features

- **🗂️ Area Isolation** – Separate digital environments for work, learning, gaming, trading, travel, math
- **🌐 Browser Profiles** – Launch Firefox profiles per area (`firefox -P {area}`)
- **📝 Dedicated Notes** – Append notes to `area/notes/my_notes.txt`
- **🧠 Flashcards** – Study decks in `area/flashcards/*.txt` (format: `front|back`)
- **⏱️ Auto Time Tracking** – Sessions recorded to `sessions.csv`, stats with `iceland stats`
- **🎮 TUI Selector** – Interactive area switching with `iceland tui`
- **♻️ Reset Commands** – `destroy <area> browser|notes` clears clutter instantly
- **⚡ 100% Local** – No cloud, no telemetry, all data in `~/.iceland/`
- **🔧 Extensible** – Add custom areas with `add-area <name>`

---
![📝 Testing Your Installation](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=📝Testing&fontColor=000000&fontSize=22)


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
![🌐 Suggested Links Per Area](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=🌐Suggested_Links_Per_Area&fontColor=000000&fontSize=22)


Each ICEland area contains a `links.txt` file with curated resources
to match that specific focus environment.

These links are automatically shown when switching areas
and help you enter the right mindset instantly.

![Section Beige](https://capsule-render.vercel.app/api?type=rect&color=F5F5DC&height=60&section=header&text=%F0%9F%8E%AE%20Gaming%20Area&fontColor=333333&fontSize=22)

Example links you might include:

- 🕹️ Addictive browser games  
- 🧩 Quick mini-games  
- 🍿 "Snack" games for short breaks  

The goal: controlled fun without drifting into chaos.

---

![Section Beige](https://capsule-render.vercel.app/api?type=rect&color=F5F5DC&height=60&section=header&text=%F0%9F%93%88%20Trading%20Area&fontColor=333333&fontSize=22)

Example curated resources:

- Forex news and economic calendar from **ForexFactory**  
- Market analysis tools  
- Currency strength dashboards  

This keeps you focused only on trading-related information.

---

![Section Beige](https://capsule-render.vercel.app/api?type=rect&color=F5F5DC&height=60&section=header&text=%F0%9F%93%9A%20Learning%20Area&fontColor=333333&fontSize=22)

Example resources:

- 🎓 Your university portal (e.g., Primus)  
- 📖 Online course platforms  
- 📑 Study reference websites  
- 🧠 Research databases  

Everything in one isolated learning zone.

---

![Section Beige](https://capsule-render.vercel.app/api?type=rect&color=F5F5DC&height=60&section=header&text=%F0%9F%92%BC%20Work%20Area&fontColor=333333&fontSize=22)

- Internal dashboards  
- Project management tools  
- Documentation systems  
- Company portals  

Zero distractions.

---
![Section Beige](https://capsule-render.vercel.app/api?type=rect&color=F5F5DC&height=60&section=header&text=%E2%9E%97%20Math%20Area&fontColor=333333&fontSize=22)

- Symbolic calculators  
- Formula references  
- Math problem solvers  
- Online textbooks  

---

![Section Beige](https://capsule-render.vercel.app/api?type=rect&color=F5F5DC&height=60&section=header&text=%E2%9C%88%EF%B8%8F%20Traveling%20Area&fontColor=333333&fontSize=22)

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
![Section Grey](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=%F0%9F%93%9A%20Flashcards&fontColor=000000&fontSize=22)

🧠 What is a deck?

- A **deck** is a single `.txt` file.
- It lives inside the area’s `flashcards` folder:

```text
~/.iceland/<area>/flashcards/<deck_name>.txt
Each line in that file is one card.
```
Structure:
```
Text before | → card front (question / prompt).

Text after | → card back (answer / explanation).

Lines without | are skipped with a warning.
```
1️⃣ Create your first deck (math example)
```bash
# Make sure the folder exists
mkdir -p ~/.iceland/math/flashcards
```
# Create a new deck file
```
cd ~/.iceland/math/flashcards
nano algebra_basics.txt
Inside algebra_basics.txt, add one card per line:
```
```text
What is 2+2?|4
Derivative of x²|2x
Integral of 2x|x² + C
Solve 3x + 2 = 11|x = 3
Save and exit.
```
2️⃣ Study the deck
Use the flashcards command for that area:

```bash
iceland flashcards math
```
You will see:

A list of available decks in ~/.iceland/math/flashcards/.

Use ↑ / ↓ to select a deck, Enter to confirm.

For each card:

ICEland shows the front.

Press Enter to reveal the back.

Press Enter again to move to the next card.

When all cards are done, ICEland prints “Finished deck”.

3️⃣ Decks for any area
You can repeat the same pattern for any area:

```bash
mkdir -p ~/.iceland/learning/flashcards
cd ~/.iceland/learning/flashcards
nano rust_basics.txt
```
Example rust_basics.txt:

text
What is ownership in Rust?|A set of rules that governs how a Rust program manages memory
What is borrowing?|Accessing data without taking ownership
What does &mut mean?|A mutable reference
Then:

```bash
iceland flashcards learning
```
4️⃣ Tips for powerful decks

Keep front short and clear (“What is…?”, “Define…”, “Example of…”).

Put only one idea per card.

Use the same deck file to grow your knowledge over time.

Create separate decks per topic: algebra_basics.txt, rust_ownership.txt, german_B1_vocab.txt, etc.

Flashcards live entirely in ~/.iceland, so everything stays local and under your control.

```text
undefined
```
Study:

```bash
iceland flashcards math
```

---
![Section Grey](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=%E2%8F%B1%EF%B8%8F%20Time%20Tracking&fontColor=000000&fontSize=22)

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

![Section Grey](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=%F0%9F%96%A5%EF%B8%8F%20Browser%20Integration&fontColor=000000&fontSize=22)

Example:

```bash
firefox -P math &
```



All ICEland data lives locally:

```text
~/.iceland/
```

No servers. No accounts. No tracking.

---
🎯 Designed for You
ICEland adapts to your unique workflow — whatever your focus, we've got you covered.

🎓
University Students
Need separated spaces for studying, projects, and leisure on the same laptop so they don't mix "exam prep" with gaming or social media.

⌨️
Developers Who Live in the Terminal
Prefer a CLI and Git-friendly workflow, want per-project or per-topic environments (work, learning, side projects) with minimal overhead.

📚
Self-Learners & Career Changers
Study multiple topics in parallel (e.g., Rust, math, German) and want each topic to have its own notes, flashcards, links, and time tracking.

💼
Remote Workers & Freelancers
Switch between clients or roles during the day and need clear context boundaries (client A vs client B vs personal tasks) without extra SaaS tools.

⏱️
Focus & Productivity Enthusiasts
Use time blocking or deep-work methods and want a lightweight, local tool to log sessions per area and review where their time actually went.

🔒
Privacy-Conscious Users
Prefer local-only tools with no accounts, cloud sync, or telemetry, but still want structure: organized folders, logs, and repeatable routines.

✨ One tool. Endless possibilities.

![Section Grey](https://capsule-render.vercel.app/api?type=rect&color=D3D3D3&height=60&section=header&text=%F0%9F%93%84%20License&fontColor=000000&fontSize=22)


Built for educational purposes and personal productivity.

---

**Separate your worlds. Sharpen your focus.**  
ICEland — From chaos to clarity.
