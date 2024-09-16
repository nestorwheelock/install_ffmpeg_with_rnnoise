
# FFmpeg and RNNoise Installer

This Rust-based script automates the installation of FFmpeg with RNNoise support on Unix-like systems. It installs necessary dependencies, clones and builds both RNNoise and FFmpeg, and verifies that the installation was successful.

## Features

- **Automates FFmpeg Installation**: Installs FFmpeg from source with support for various codecs and filters.
- **RNNoise Integration**: Builds and installs RNNoise to enable advanced noise suppression in FFmpeg.
- **Dependency Management**: Automatically installs all required libraries and dependencies for building FFmpeg and RNNoise.
- **Verifies Installation**: Ensures that FFmpeg is installed correctly and that the RNNoise filter is available.

## Requirements

- **Operating System**: Unix-like systems (Linux/macOS). The script uses `apt-get` to install dependencies, so it is suitable for Debian-based systems.
- **Rust Toolchain**: The script is written in Rust, so you will need to install Rust to compile and run the program.

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/your-username/ffmpeg-rnnoise-installer.git
cd ffmpeg-rnnoise-installer
```

### Step 2: Build the Installer

```bash
cargo build --release
```

### Step 3: Run the Installer

```bash
./target/release/ffmpeg_installer
```

The script will:

1. Install all required dependencies for building FFmpeg and RNNoise.
2. Clone and build the RNNoise library.
3. Clone and build FFmpeg from source with support for RNNoise and other popular codecs.
4. Verify that FFmpeg is installed and the RNNoise filter (`arnndn`) is available.

### Example Output

```bash
Starting FFmpeg installation script...
Installing dependencies...
Cloning and building RNNoise...
Configuring FFmpeg...
Compiling FFmpeg...
Installing FFmpeg...
Verifying FFmpeg installation...
FFmpeg version ...
Checking for RNNoise filter...
RNNoise filter is available.
FFmpeg installation completed successfully.
```

## License

This project is licensed under the GNU GPLv3. See the [LICENSE](LICENSE) file for more details.

## TODO

-- Make it compatible with FreBSD and other non-debian distros.
