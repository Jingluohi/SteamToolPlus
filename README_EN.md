# SteamToolPlus User Guide

> **Version**: v1.01 &nbsp;|&nbsp; **.NET 10.0**

---

## 📑 Contents

[Quick Start](#-quick-start) · [Patch Injection](#-patch-injection) · [Save Configuration](#-save-configuration) · [LAN Multiplayer](#-lan-multiplayer) · [Achievement Statistics](#-achievement-statistics) · [Advanced Features](#-advanced-features) · [FAQ](#-faq)

---

## 🚀 Quick Start

**Step 1**: Select game card on main interface → Enter details page

**Step 2**: Click [Select Patch Source] → Choose Goldberg Emulator folder

**Step 3**: Click [Browse] → Choose game installation directory

**Step 4**: Click [💉 Inject Patch] → Wait for completion

---

## 🎮 Patch Injection

### Method 1: Special Game Patches (Recommended)

**For**: Popular games (Elden Ring, Monster Hunter, etc.)

**Steps**:
1. Select game card on main interface
2. Confirm patch type (LAN/Steam/Steamless/Denuvo)
3. Click [💉 Inject Patch]
4. Select patch source folder
5. Select game installation directory
6. Wait for injection to complete

**Patch Types**:
| Type | Description |
|------|-------------|
| 🌐 LAN Multiplayer | Multiplayer within same network |
| 🌍 Steam Multiplayer | Requires legitimate Steam client |
| 🚫 Steamless Launch | Runs completely offline |
| 🔒 Denuvo Game | For Denuvo encrypted games |

---

### Method 2: Basic Configuration (Universal)

**For**: Any game that supports Goldberg

#### 1️⃣ Select Emulator Mode

| Mode | Features |
|------|----------|
| **Steamless Launch** | Offline, no Steam required, for single-player |
| **LAN Multiplayer** | Local multiplayer, requires network config |
| **Steam Multiplayer** | Requires Steam client, online multiplayer |

#### 2️⃣ Configure Paths

| Path Item | Description | Example |
|-----------|-------------|---------|
| ddv20.exe | Steam download tool | `Resources/ddv20.exe` |
| Game Manifest Folder | Contains game.json, manifest, etc. | Download from ManifestHub |
| Game Save Directory | Location for downloaded games | `D:\Games\` |

#### 3️⃣ Configure Parameters

**Steam AppID**: Query game ID on [SteamDB](https://steamdb.info/)

**DLC Configuration**: Click [Unlock All DLC with One Click] or manually configure subscription groups

#### 4️⃣ One-Click Configuration

Click [⚡ Complete All Configuration] → Automatically generate config files

#### 5️⃣ Inject Patch

1. Click [💉 Inject Patch]
2. Select patch source folder
3. Select game installation directory
4. Wait for progress bar to complete

**Success Indicators**:
- ✅ Log shows "Injection complete"
- ✅ Patch files appear in game directory
- ✅ Game can run normally

---

## 💾 Save Configuration

### Select Save Mode

| Mode | For Users | Save Location |
|------|-----------|---------------|
| Default Save Path | Beginners | `%appdata%\GSE Saves\` |
| Portable Save | Most players | `\Saves` in game directory |
| Custom Path | Advanced users | User-specified location |

### Steps

**1. Select Mode**: Click corresponding radio button

**2. Custom Path** (Custom mode only):
   - Click [Browse]
   - Select/create folder
   - Recommended structure:
```
D:\GameSaves\
├── TheWitness\
├── Cyberpunk2077\
└── OtherGame\
```

**3. Save Configuration**: Click [⚡ One-Click Save Configuration]

### Backup and Restore

**Backup Save**:
1. Click [Backup Save]
2. Select save folder
3. Select backup save location
4. Automatically creates compressed package

**Restore Save**:
1. Click [Restore Save]
2. Select backup file
3. Select restore target location
4. Automatically decompresses and restores

---

## 🌐 LAN Multiplayer

### Preparation

**Network Environment**:
- ✅ All computers connected to same network
- ✅ Can ping each other
- ✅ Disable firewall or add exceptions

**Game Version**:
- ✅ All computers install same version
- ✅ Use same patch version
- ✅ Game language settings consistent

### Configuration Steps

**1. Basic Configuration**
- Enter [LAN Multiplayer] tab
- Select game directory and executable
- Set Steam AppID

**2. Network Configuration**

**Subscribed Groups**
- One Group ID per line
- Used to identify players in same room

**Groups Clans**
- Format: `groupid|name|tag`
- Example:
```
1234567890|My Clan|MC
0987654321|Friends|FR
```

**Custom Network Segments** (to solve host not found)
```
192.168.1.0/24
192.168.0.0/24
10.0.0.0/24
```

**3. One-Click Configuration**: Click [⚡ One-Click LAN Configuration]

**4. Inject Patch**: Repeat injection on all computers participating in multiplayer

**5. Launch Game**
- Host launches first and creates room
- Other players launch and search for room
- Join game and start multiplayer

### Firewall Configuration

**Method 1: Automatic** (Recommended)
- Program automatically adds firewall rules
- Click "Allow Access" if prompted

**Method 2: Manual**
1. Open Windows Firewall
2. Click "Allow app through firewall"
3. Add game executable
4. Check "Private" and "Public" networks

**Ports to Allow**:
- UDP 27015-27030 (game ports)
- UDP 4380 (voice chat)

---

## 🏆 Achievement Statistics

### Configuration Steps

**1. Select Achievement File**
- Click [Select Achievement File]
- Choose `achievements.json`
- Automatically loads achievement list

**2. View Achievements**
- Unlocked: Green display
- Locked: Gray display
- Click to view details

**3. Manually Unlock Achievements**
- Enter achievement ID
- Fill in name and description (optional)
- Click [Add Achievement]
- Achievement unlocks immediately

**4. One-Click Configuration**: Click [⚡ One-Click Achievement Configuration]

### Achievement File Format

```json
{
  "achievements": [
    {
      "name": "ACHIEVEMENT_ID",
      "title": "Achievement Name",
      "desc": "Achievement Description",
      "hidden": 0,
      "icon": "achievement_icon.jpg",
      "icon_gray": "achievement_icon_gray.jpg"
    }
  ]
}
```

**Field Descriptions**:
- `name`: Unique identifier
- `title`: Display name
- `desc`: Description text
- `hidden`: Whether hidden (0=visible, 1=hidden)
- `icon`: Unlocked icon
- `icon_gray`: Locked icon

### Methods to Unlock Achievements

**Method 1: Via Program**
1. Enter [Achievement Statistics Configuration]
2. Enter achievement ID
3. Click unlock button

**Method 2: Modify File**
1. Find `account_data.vdf`
2. Add achievement record
3. Restart game

**Method 3: Console** (if game supports)
1. Press console key in game
2. Enter unlock command
3. Achievement unlocks immediately

---

## 🔧 Advanced Features

### Configuration Items

| Feature | Description | Configuration File |
|---------|-------------|-------------------|
| SteamHTTP Simulation | Simulates Steam HTTP API responses | `configs.main.ini` |
| Custom Avatar | Set player avatar image | `account_avatar.png` |
| Language Configuration | Specify supported language list | `supported_languages.txt` |
| Mods Support | Load game mods | `mods/` folder |
| Controller Configuration | XInput controller support | `controller/` folder |
| Auth Token | EncryptedAppTicket support | `configs.user.ini` |

### Steps

**1. Configure Parameters**: Check corresponding options, fill in necessary content

**2. One-Click Configuration**: Click [⚡ One-Click Advanced Configuration]

**3. Inject Patch**: Click [💉 Inject Patch]

### Custom Avatar Configuration

**Image Requirements**:
- Format: PNG, JPG, JPEG
- Size: 64x64 pixels recommended
- Filename: `account_avatar`

**Location**:
- Global: `%appdata%\GSE Saves\settings\`
- Local: `steam_settings\` folder in game directory

---

## ❓ FAQ

### Game Won't Launch After Patch Injection

**Solutions**:
1. Check if patch matches game version
2. Check error information in log area
3. Try restoring game and re-injecting
4. Verify game file integrity

### Game Crashes or Freezes

**Solutions**:
1. Disable Overlay feature
2. Check game file integrity
3. Try other patch versions
4. Run as administrator

### Cannot Recognize Game Path

**Solutions**:
1. Confirm path doesn't contain Chinese characters
2. Check path permissions
3. Run program as administrator
4. Use English path

### Game Doesn't Recognize Save After Configuration

**Solutions**:
1. Check if save path is correct
2. Confirm save format is compatible
3. Check if game supports Goldberg
4. Restart game

### Can't Find Host in LAN Multiplayer

**Solutions**:
1. Check if all computers are on same network segment
2. Disable firewall or add exceptions
3. Configure custom network segments
4. Check if host correctly created room
5. Try manually entering IP address to connect

### Achievement Popup Doesn't Display After Unlock

**Solutions**:
1. Check if Overlay is enabled
2. Confirm achievement file configuration is correct
3. Restart game
4. Check if sound files exist

### How to Reset All Achievements

**Method**:
1. Delete `account_data.vdf`
2. Or use program's [Reset Achievements] function
3. Restart game

---

## 📞 Technical Support

**Having Issues?**
1. Check error information in log area
2. Verify configuration files are correct
3. Contact developer or community

**Related Links**:
- 📦 [ManifestHub GitHub](https://github.com/SteamAutoCracks/ManifestHub)
- 📺 [Bilibili: 鲸落_hi](https://space.bilibili.com/405707676)

---

## 📝 Changelog

### v1.01

**UI Optimization**
- ✅ Optimized game cover display (275×160)
- ✅ Unified font size (13-19)
- ✅ Optimized search bar style
- ✅ Tab highlight display

**Feature Enhancement**
- ✅ Added [One-Click Configuration] button to all feature interfaces
- ✅ Added cloud download tab
- ✅ Added ManifestHub jump button

**Performance Improvement**
- ✅ Optimized log append function (throttling)
- ✅ Reduced Dispatcher call frequency
- ✅ Used color cache to reduce object creation
- ✅ Optimized UI response speed

**Bug Fixes**
- ✅ Fixed tutorial numbering errors
- ✅ Fixed patch injection backup logic
- ✅ Fixed game restoration logic
- ✅ Fixed unpacking function path issues

---

**Thank you for using SteamToolPlus!** 🎉
