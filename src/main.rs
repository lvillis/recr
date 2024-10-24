use clap::{Arg, Command};
use std::process::Command as SystemCommand;

fn main() {
    match SystemCommand::new("docker").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("Docker is available: {}", version);
            } else {
                eprintln!("Docker command failed. Please ensure Docker is installed and in your PATH.");
                std::process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("Docker is not installed or not found in PATH.");
            std::process::exit(1);
        }
    }

    let matches = Command::new("ReCr")
        .version("0.1.0")
        .author("lvillis")
        .about("🐳 Re-tag Docker Images with Integrated Pull and Push.")
        .arg(
            Arg::new("source")
                .help("The source Docker image tag")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("target")
                .help("The target Docker image tag")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let source_image = matches.get_one::<String>("source").unwrap();
    let target_image = matches.get_one::<String>("target").unwrap();

    // Pull the source image
    SystemCommand::new("docker")
        .args(["pull", source_image])
        .status()
        .expect("Failed to pull source image");

    // Tag the source image with the target tag
    SystemCommand::new("docker")
        .args(["tag", source_image, target_image])
        .status()
        .expect("Failed to tag image");

    // Push the target image
    SystemCommand::new("docker")
        .args(["push", target_image])
        .status()
        .expect("Failed to push target image");

    println!(
        "Image {} has been pulled, tagged, and pushed as {}",
        source_image, target_image
    );
}
