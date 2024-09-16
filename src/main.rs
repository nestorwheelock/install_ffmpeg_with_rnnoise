use std::process::{Command, exit};
use std::env;
use std::path::Path;

fn main() {
    // Ensure we're using a Unix-like system (Linux or macOS)
    if cfg!(unix) {
        println!("Starting FFmpeg installation script...");

        // Step 1: Install dependencies
        install_dependencies();

        // Step 2: Clone and build RNNoise
        if !Path::new("rnnoise").exists() {
            clone_and_build_rnnoise();
        } else {
            println!("RNNoise already cloned and built.");
        }

        // Step 3: Clone and build FFmpeg
        if !Path::new("ffmpeg").exists() {
            clone_ffmpeg();
        }

        // Step 4: Configure, compile, and install FFmpeg
        configure_and_build_ffmpeg();

        // Step 5: Verification
        verify_ffmpeg_installation();

        println!("FFmpeg installation completed successfully.");
    } else {
        eprintln!("This script is intended for Unix-like systems.");
        exit(1);
    }
}

fn install_dependencies() {
    println!("Installing dependencies...");
    let output = Command::new("sudo")
        .arg("apt-get")
        .arg("install")
        .arg("-y")
        .args(&[
            "autoconf", "automake", "build-essential", "cmake", "git-core", "libass-dev",
            "libfreetype6-dev", "libgnutls28-dev", "libmp3lame-dev", "libnuma-dev",
            "libopus-dev", "libtheora-dev", "libtool", "libvorbis-dev", "libvpx-dev",
            "pkg-config", "texinfo", "wget", "yasm", "zlib1g-dev", "libunistring-dev",
            "libfdk-aac-dev", "libx264-dev", "libx265-dev",
        ])
        .output()
        .expect("Failed to install dependencies");

    if !output.status.success() {
        eprintln!("Failed to install dependencies.");
        exit(1);
    }
}

fn clone_and_build_rnnoise() {
    println!("Cloning and building RNNoise...");
    Command::new("git")
        .args(&["clone", "https://github.com/xiph/rnnoise.git"])
        .status()
        .expect("Failed to clone RNNoise repository");

    env::set_current_dir("rnnoise").expect("Failed to change directory to rnnoise");

    Command::new("./autogen.sh")
        .status()
        .expect("Failed to run autogen.sh");

    Command::new("./configure")
        .status()
        .expect("Failed to configure RNNoise");

    Command::new("make")
        .status()
        .expect("Failed to build RNNoise");

    Command::new("sudo")
        .args(&["make", "install"])
        .status()
        .expect("Failed to install RNNoise");

    Command::new("sudo")
        .arg("ldconfig")
        .status()
        .expect("Failed to run ldconfig");

    env::set_current_dir("..").expect("Failed to change directory back");
}

fn clone_ffmpeg() {
    println!("Cloning FFmpeg...");
    Command::new("git")
        .args(&["clone", "https://git.ffmpeg.org/ffmpeg.git", "ffmpeg"])
        .status()
        .expect("Failed to clone FFmpeg repository");
}

fn configure_and_build_ffmpeg() {
    println!("Configuring FFmpeg...");
    env::set_current_dir("ffmpeg").expect("Failed to change directory to ffmpeg");

    let configure_output = Command::new("./configure")
        .env("PKG_CONFIG_PATH", "/usr/local/lib/pkgconfig")
        .args(&[
            "--enable-gpl", "--enable-libx264", "--enable-libx265", "--enable-nonfree",
            "--enable-libfreetype", "--enable-libmp3lame", "--enable-libopus", "--enable-libvpx",
            "--enable-libvorbis", "--enable-libass", "--enable-libfdk-aac",
        ])
        .output()
        .expect("Failed to configure FFmpeg");

    if !configure_output.status.success() {
        eprintln!("FFmpeg configuration failed.");
        eprintln!("{}", String::from_utf8_lossy(&configure_output.stderr));
        exit(1);
    }

    println!("Compiling FFmpeg...");
    let make_output = Command::new("make")
        .arg(format!("-j{}", num_cpus::get()))
        .output()
        .expect("Failed to build FFmpeg");

    if !make_output.status.success() {
        eprintln!("FFmpeg compilation failed.");
        eprintln!("{}", String::from_utf8_lossy(&make_output.stderr));
        exit(1);
    }

    println!("Installing FFmpeg...");
    let install_output = Command::new("sudo")
        .arg("make")
        .arg("install")
        .output()
        .expect("Failed to install FFmpeg");

    if !install_output.status.success() {
        eprintln!("FFmpeg installation failed.");
        eprintln!("{}", String::from_utf8_lossy(&install_output.stderr));
        exit(1);
    }

    env::set_current_dir("..").expect("Failed to change directory back");
}

fn verify_ffmpeg_installation() {
    println!("Verifying FFmpeg installation...");

    let ffmpeg_version_output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .expect("Failed to verify FFmpeg installation");

    if !ffmpeg_version_output.status.success() {
        eprintln!("FFmpeg verification failed.");
        exit(1);
    }

    println!("{}", String::from_utf8_lossy(&ffmpeg_version_output.stdout));

    println!("Checking for RNNoise filter...");
    let ffmpeg_filters_output = Command::new("ffmpeg")
        .arg("-filters")
        .output()
        .expect("Failed to check FFmpeg filters");

    if String::from_utf8_lossy(&ffmpeg_filters_output.stdout).contains("arnndn") {
        println!("RNNoise filter is available.");
    } else {
        eprintln!("RNNoise filter is not available. Please check the installation.");
        exit(1);
    }
}

