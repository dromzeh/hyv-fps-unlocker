# hyv-fps-unlocker

Lightweight program for Windows that allows you to modify the FPS of **Honkai Impact 3rd** and **Honkai: Star Rail** by changing registry values.

Read the blog post about this project (For Hi3 unlocking) [here](https://dromzeh.dev/posts/hi3-fps-unlock/).

> **Note**:
> Because you're modifying the registry, changing your options in-game **will overwrite the fps to the value you set in-game. To change the fps again, you will have to re-run the program.**

> **Warning**:
> **I am not responsible for any consequences that may occur from using this program.** Use at your own risk.

## Download & Usage

## From the releases page

- Download the latest release from [the releases page](https://github.com/dromzeh/hyv-fps-unlocker/releases)
- Run `hyv-fps-unlocker.exe` and follow the instructions.

## Building

- `git clone https://github.com/dromzeh/hyv-fps-unlocker`
- `cargo build --release` (Assuming you have [Rust](https://rustup.rs/) installed)
- Run the built executable in `./target/release/hyv-fps-unlocker.exe`.

**You don't have to run the program as Administrator**, if you're having issues, you can try running it as Administrator.

## Known Issues

### Registry Issues [Can't be fixed]

- Modifying the registry values **will NOT work if you have the game open.**
- The game **will overwrite the registry values if you change the FPS in-game.** 
- **Options menus for games may not work as intended** after modifying the registry values.

## Contributing

Pull requests are welcome. For major changes, please open an issue first.

## License

hyv-fps-unlocker is licensed under [MIT](https://mit.dromzeh.dev/) - see [LICENSE](LICENSE) for more information.
