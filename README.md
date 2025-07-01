# hyv-fps-unlocker

A Windows registry modifier that unlocks FPS limits for **Honkai Impact 3rd** and **Honkai: Star Rail**.

![Windows](https://img.shields.io/badge/os-windows-blue)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/dromzeh/hyv-fps-unlocker/main)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/dromzeh/hyv-fps-unlocker/build.yml)

## How It Works

This unlocker modifies registry values where the games store their graphics settings, allowing you to set FPS values beyond what's available in the in-game menus. It automatically detects which games you have installed.

## Usage

1. Close the game completely
2. Download and run `hyv-fps-unlocker.exe` from [releases](https://github.com/dromzeh/hyv-fps-unlocker/releases)
3. Select the game and desired FPS value
4. Launch the game

## Fixes For Known Issues

- If the unlocker doesn't detect your game, launch it and change any graphics setting to create the registry entries. The easiest option is changing your resolution, applying it, then changing it back.

- If your in-game graphics menu appears broken, simply change any graphics setting (e.g resolution) to restore the menu to normal.

- When you modify graphics settings in-game, the game will replace your FPS values, requiring you to rerun the unlocker after each change.

## Building from Source

Requires [Rust](https://rustup.rs/) and [Git](https://git-scm.com/) installed:

```bash
git clone https://github.com/dromzeh/hyv-fps-unlocker
cd hyv-fps-unlocker
cargo build --release
```

The executable will be in `./target/release/hyv-fps-unlocker.exe`.

## Contributing

Pull requests welcome. For major changes, please open an issue first.

## Disclaimer

Use at your own risk. This tool modifies system registry values.

## License

[MIT License](LICENSE)
