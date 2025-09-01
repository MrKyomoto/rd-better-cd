# rd-better-cd: Modern TUI Directory Changer
rd-better-cd(`rd`) is a terminal tool that replaces the traditional `cd` command with a visual Text-based User Interface (TUI). It simplifies directory navigation via keyboard controls (including Vim-style shortcuts) and a modern interface, enabling one-click directory switching in Bash.


![showcase](/assets/showcase.gif)
## Features
- **Intuitive TUI**: Clean and modern interface for visual directory browsing.
  - **Preview** sub file's or dir's contents
- **Keyboard Controls**: Supports both standard arrow keys and Vim shortcuts (hjkl).
- **Hidden Files Toggle**: Easily show/hide dot-files (e.g., `.git`, `.bashrc`) with a single key.
- **Bash Integration**: Automatically updates the Bash working directory after selection (requires a simple function setup).


## Keyboard Shortcuts
| Keys              | Function                                 |
|-------------------|------------------------------------------|
| `ESC` / `←` / `h` | Go back to the parent directory.         |
| `Enter` / `→` / `l` | Enter the selected subdirectory.        |
| `↑` / `k`         | Move highlight up.                       |
| `↓` / `j`         | Move highlight down.                     |
| `H` (uppercase)   | Toggle visibility of dot-files/directories. |
| `Space`           | Confirm and switch Bash to the selected directory. |
| `q` / `Q`         | Quit the TUI (Bash directory remains unchanged). |


## Step I - Installation

### Method 1: Download Release (Recommended)
1. Go to the [Releases page](https://github.com/MrKyomoto/rd-better-cd/releases).
2. Download the latest release for your system (e.g., `rd-better-cd-linux-amd64.tar.gz`).
3. Extract the file:
```bash
tar -zxvf rd-better-cd-*.tar.gz
```
4. Move the binary to a system path (e.g., /usr/local/bin)
```bash
sudo mv rd-better-cd /usr/local/bin/
```

### Method 2: Build from Source
- This part is on your own
- Basically all you need is:
```bash
git clone https://github.com/MrKyomoto/rd-better-cd.git
cd rd-better-cd
cargo build --release
```
- And then put the binary into `/usr/local/bin/`
- Remaining part is just like Method 1

## Step II - Bash Integration (Required)
- To auto-switch the Bash directory, add this function to ~/.bashrc (or ~/.zshrc for Zsh):
```bash
rd() {
    # please replace it to your actual path
    RATATUI_BETTER_CD="rd-better-cd"
    TEMP_FILE="/tmp/rd_selected_dir"
    rm -f "$TEMP_FILE"
    "$RATATUI_BETTER_CD"
    if [ -f "$TEMP_FILE" ]; then
        SELECTED_DIR=$(cat "$TEMP_FILE" | tr -d '\r\n')
        rm -f "$TEMP_FILE"
        if [ -d "$SELECTED_DIR" ]; then
            cd "$SELECTED_DIR" || echo "Failed to cd: $SELECTED_DIR"
            echo "Current dir: $(pwd)"
        else
            echo "Invalid dir: $SELECTED_DIR"
        fi
    else
        echo "No directory selected."
    fi
}
```

- Apply the changes:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

## Usage
1. Launch: rd
2. Navigate with arrow keys/Vim keys (hjkl) (ESC or Enter are also supported)
3. Press Space to switch to the selected directory
4. Press H to toggle hidden files(starts with dot '.')
5. Press q/Q to quit (select current root directory)

## Known Issues
- Git-related error: Entering .git directories may show a fatal error. This is harmless and does not affect functionality.
