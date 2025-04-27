use std::process;

use anyhow::Result;
use clap::{ArgMatches, Command};

const SERVICE_NAME: &str = "{{ project-name }}";

fn main() -> Result<()> {
    let args = clap::command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("install").about("Cargo Install"))
        .subcommand(
            Command::new("docker")
                .about("Docker Operations")
                .subcommand(Command::new("build").about("Builds an application Docker image."))
                .subcommand(Command::new("rmi").about("Removes the application Docker image.")),
        )
        .get_matches();

    match args.subcommand() {
        Some(("docker", args)) => handle_docker_commands(args),
        Some(("install", args)) => handle_install_command(args),
        Some((command, _)) => anyhow::bail!("Unexpected command: {command}"),
        None => anyhow::bail!("Expected subcommand"),
    }
}

fn handle_install_command(_args: &ArgMatches) -> Result<()> {
    let mut command = process::Command::new("cargo");
    command
        .args(["install", "--path", "crates/{{project-name}}-bin"])
        .status()?;

    Ok(())
}

fn handle_docker_commands(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("build", _args)) => docker_build(),
        Some(("rmi", _args)) => docker_rmi(),
        _ => Ok(()),
    }
}

fn docker_build() -> Result<()> {
    process::Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(SERVICE_NAME)
        .arg(".")
        .spawn()?
        .wait()?;

    Ok(())
}

fn docker_rmi() -> Result<()> {
    process::Command::new("docker")
        .arg("rmi")
        .arg(SERVICE_NAME)
        .spawn()?
        .wait()?;

    Ok(())
}
