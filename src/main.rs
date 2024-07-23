use clap::{Arg, Command};
use std::process::Command as SystemCommand;

fn main() {
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
