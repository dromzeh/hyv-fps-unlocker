# hyv-fps-unlocker

Modifies registry entries for **Honkai Impact 3rd** and **Honkai: Star Rail**, unlocking the FPS cap.

![Windows](https://img.shields.io/badge/os-windows-blue)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/dromzeh/hyv-fps-unlocker/main)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/dromzeh/hyv-fps-unlocker/build.yml)

---

## Download & Usage

### From the Releases Page

- Download the latest release from [the releases page](https://github.com/dromzeh/hyv-fps-unlocker/releases).
- Run `hyv-fps-unlocker.exe`.

## Building from Source

- Clone the repository using `git clone https://github.com/dromzeh/hyv-fps-unlocker`.
- Build the project using `cargo build --release` (assuming you have [Rust](https://rustup.rs/) installed).
- Launch the compiled executable located in `./target/release/hyv-fps-unlocker.exe`.

## Known Issues

### Can't be fixed

- The game will overwrite the modified FPS value(s) if you update settings in-game, requiring you to run the unlocker again.
- Options menus in-game may not function as expected.

## Broken Menu Fix

If you're facing issues with the unlocker, such as broken settings menus, you can fix this by just changing any setting in-game and restarting. This WILL disable the FPS limit set by the unlocker, requiring you to run it again.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Note

I am not responsible for any damage caused to your system. Please use this program at your own risk!

## License

**hyv-fps-unlocker** operates under the [MIT License](https://mit.dromzeh.dev/). Refer to [LICENSE](LICENSE).
