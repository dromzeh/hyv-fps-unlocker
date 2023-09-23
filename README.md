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

### Registry Method Limitations

- Altering registry values will be ineffective if the game is currently running.
- The game will overwrite the modified registry value(s) if you update settings in-game.
- Options menus in-game may not function as expected.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Note 

I am not responsible for any damage caused to your system. Please use this program at your own risk.

## License

**hyv-fps-unlocker** operates under the [MIT License](https://mit.dromzeh.dev/). Refer to [LICENSE](LICENSE).
