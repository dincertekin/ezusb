# ezusb

ezusb is a USB disk writer tool that allows you to write files to USB disks easily.  
And i'm trying to make it cross-platform.

## Usage

🪟 On Windows, run:

```sh
.\ezusb.exe
```

🐧🍎 On Linux and macOS, run:

```sh
./ezusb
```

And menu will come!

## Build

🪟 On Windows, run:

```sh
cargo build --target x86_64-pc-windows-gnu
```

🐧 On Linux, run:

```sh
cargo build --target x86_64-unknown-linux-gnu
```

🍎 On macOS, run:

```sh
cargo build --target x86_64-apple-darwin
```

## Notes

- Currently it uses dd utility on all platforms to write disks.
