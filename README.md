# Strom

Strom is a fast, concurrent Rust-based Text User Interface (TUI) application designed to convert `.m4b` audiobook files into `.mp3` files.

## Features
* **Interactive TUI:** Built with `ratatui` for an intuitive, keyboard-driven interface.
* **Two Conversion Modes:**
  * **Single File:** Convert an entire `.m4b` file into a single `.mp3`.
  * **Split by Chapter:** Automatically extract and split the audiobook into multiple `.mp3` files based on embedded chapter metadata.
* **High Performance:** Uses `tokio` for concurrent conversions, maximizing CPU utilization when processing multiple chapters or files.

## Prerequisites
You must have the following installed on your system:
* [Rust](https://www.rust-lang.org/tools/install)
* [FFmpeg](https://ffmpeg.org/) (specifically `ffmpeg` and `ffprobe` must be available in your `$PATH`)

## Usage
1. Run `cargo run` in a directory containing `.m4b` files.
2. Use the arrow keys to navigate the list.
3. Press `Space` to select files.
4. Press `Enter` to start conversion.
5. Follow the prompt to select your desired conversion mode.
