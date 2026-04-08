# NEKOWALL

```
  ███╗   ██╗███████╗██╗  ██╗ ██████╗ ██╗    ██╗ █████╗ ██╗     ██╗
  ████╗  ██║██╔════╝██║ ██╔╝██╔═══██╗██║    ██║██╔══██╗██║     ██║
  ██╔██╗ ██║█████╗  █████╔╝ ██║   ██║██║ █╗ ██║███████║██║     ██║
  ██║╚██╗██║██╔══╝  ██╔═██╗ ██║   ██║██║███╗██║██╔══██║██║     ██║
  ██║ ╚████║███████╗██║  ██╗╚██████╔╝╚███╔███╔╝██║  ██║███████╗███████╗
  ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚═╝  ╚═╝╚══════╝╚══════╝
```

**Random anime wallpaper changer powered by the [Nekosia API](https://nekosia.cat).**

Cross-platform: Windows, macOS, Linux, Android/Termux (GNOME, KDE, XFCE, MATE, Cinnamon, feh, sway, hyprland).
No external dependencies — pure Python 3.8+ standard library.

***

## Installation

```bash
pip install nekowall
```

That's it. The `nekowall` command is available immediately in any terminal.

***

## Usage

```
nekowall                        random category, random image
nekowall -c catgirl             force a specific category
nekowall -c maid --keep         keep the downloaded image file permanently
nekowall --list                show all available categories with commands
nekowall --no-banner           skip the ASCII banner
nekowall --version             show version and exit
nekowall -h                    full help
```

### Examples

```bash
# Set a random wallpaper from a random category
nekowall

# Always pick from the foxgirl category
nekowall -c foxgirl

# Pick a random one but save the image to keep it
nekowall --keep

# See every available category and the exact command to use it
nekowall --list
```

***

## How it works

1. **Loads categories** — starts with 80+ hardcoded categories from the
   [Nekosia documentation](https://nekosia.cat/documentation) and the Booru tag list,
   then fetches live tags from the API and merges any new ones with `count >= 15`.
2. **Picks a random category** (or uses the one you specified with `-c`).
3. **Fetches a random image** from `GET /api/v1/images/{category}`.
4. **Downloads the image** to `$TEMP/nekowall_{id}.{ext}` with a live progress bar.
5. **Sets the wallpaper** — Windows uses `SystemParametersInfoW` (no admin), macOS uses `osascript`, Linux uses desktop-specific tools.

***

## Available categories

The full list is always up to date — run `nekowall --list` to see everything.
A non-exhaustive sample:

| Group          | Categories                                                                  |
|---------------|----------------------------------------------------------------------------|
| Animal girls  | `catgirl` `foxgirl` `wolf-girl` `wolfgirl` `doggirl` `animal-ears`          |
| Clothes       | `maid` `maid-uniform` `uniform` `sailor-uniform` `hoodie` `gym-uniform`     |
| Accessories  | `ribbon` `headphones` `thigh-high-socks` `knee-high-socks` `white-tights`   |
| Hair         | `white-hair` `blue-hair` `pink-hair` `long-hair` `twintails` `blonde`       |
| Eyes         | `blue-eyes` `red-eyes` `purple-eyes` `heterochromia` `green-eyes`           |
| Poses & vibes| `wink` `w-sitting` `lying-down` `cute` `vtuber` `blue-archive` `hololive`   |
| Seasonal     | `valentine` `halloween` `winter` `summer` `sakura`                          |

***

## Requirements

- Python 3.8 or higher
- A supported desktop environment: Windows, macOS, GNOME, KDE, XFCE, MATE, Cinnamon, feh, sway, hyprland, Android/Termux
- Internet connection

***

## Tip: run on startup

To get a new wallpaper every time you log in:

**Windows:**
1. Press `Win + R`, type `shell:startup`, press Enter
2. Create a shortcut to `nekowall.bat` (or `nekowall`) in that folder

**macOS:**
```bash
# Create a LaunchAgent
mkdir -p ~/Library/LaunchAgents
cat > ~/Library/LaunchAgents/com.nekowall.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key><string>com.nekowall</string>
    <key>ProgramArguments</key><array><string>/usr/local/bin/nekowall</string></array>
    <key>RunAtLoad</key><true/>
</dict>
</plist>
EOF
launchctl load ~/Library/LaunchAgents/com.nekowall.plist
```

**Linux (GNOME):**
```bash
gnome-session-properties
```
Add a new entry pointing to `nekowall`.

**Android/Termux:**
Use Termux:Boot or create a startup script in Termux.

***

## Android Setup

nekowall supports Android via Termux! Here's how to set it up:

### 1. Install Termux:API

1. Install **Termux** from [F-Droid](https://f-droid.org/) (recommended over Google Play for latest version)
2. Install **Termux:API** from F-Droid as well

### 2. Install termux-api package

Open Termux and run:
```bash
pkg update && pkg upgrade
pkg install termux-api
```

### 3. Set up storage access (for --keep feature)

To save wallpapers to `/sdcard/Pictures/nekowall/` (visible in Android Gallery):
```bash
termux-setup-storage
```
Grant the requested permission when prompted.

### 4. Install nekowall

```bash
pip install nekowall
```

### 5. Run nekowall

```bash
nekowall
nekowall --keep  # Save to Gallery
```

**Features on Android:**
- Sets wallpaper using `termux-wallpaper`
- `--keep` saves to `/sdcard/Pictures/nekowall/` (visible in Gallery)
- Automatically applies "Nekosia" theme to your Termux terminal

**Note:** If you get permission errors with `--keep`, make sure to run `termux-setup-storage`.

***

## API credit

Images are provided by the **[Nekosia API](https://nekosia.cat)** — a free, public anime image API
with safety filters. No API key required.

***