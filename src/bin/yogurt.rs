extern crate clap;

extern crate yogurt;

use std::env;
use std::process::Command;
use std::error::Error;
use std::path::*;

use clap::{App, Arg, SubCommand, AppSettings};

use yogurt::{Platform, Config, Yogurt};


#[cfg(target_os = "macos")]
static PLATFORM: Platform = Platform::MacOS;
#[cfg(target_os = "linux")]
static PLATFORM: Platform = Platform::Linux;
#[cfg(target_os = "windows")]
static PLATFORM: Platform = Platform::Windows;


fn get_regular_local_storage() ->  Option<PathBuf> {
    match env::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(".yogurt");
            Some(home_dir)
        },
        None => None
    }
}

fn get_dev_local_storage() -> Option<PathBuf> {
    match std::env::current_dir() {
        Ok(mut path) => {
            path.push("dev-storage");
            Some(path)
        },
        Err(error) => None
    }
}

fn main() {
    let is_dev_environment = match std::env::var("DEV") {
        Ok(_) =>  true,
        Err(_) => false
     };

    let config = if is_dev_environment {
        Config {
            platform: &PLATFORM,
            base_url: String::from("https://localhost:4040/"),
            local_storage_dir: get_dev_local_storage().expect("Could not find current_dir() for local storage")
        }
    } else {
        Config {
            platform: &PLATFORM,
            base_url: String::from(""),
            local_storage_dir: get_regular_local_storage().expect("Could not find local storage")
        }
    };

    println!("DEV: {}", is_dev_environment);

    let version_string = "0.1.0";

    let app = clap::App::new("Yogurt")
        .bin_name("yogurt")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(version_string)
        .about("The Pharo toolchain installer")

        // VM commands
        .subcommand(SubCommand::with_name("vm")
            .about("VM related features")
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("list")
                .about("list VMs")
                .arg(Arg::with_name("location")
                    .help("locally installed VMs or available remote ones")
                    .required(true)
                    .groups(&["local", "remote"])))

            .subcommand(SubCommand::with_name("install")
                .setting(AppSettings::ArgRequiredElseHelp)
                .about("install a VM")
                .arg(Arg::with_name("version")
                    .help("The version to install"))))

        // Image commands
        .subcommand(SubCommand::with_name("image")
            .about("Image related features"));

        // Running commands
        // Use the shims mechanism of RVM

        // Params processing
    let matches = app.get_matches();
    if let Some(command) = matches.subcommand {
        match command.name.as_ref() {
            "vm" => {
                // We can safely unwrap as Clap should have validated the input
                match command.matches.subcommand.unwrap().name.as_ref() {
                    "list" => println!("list VMs"),
                    "install" => println!("Install VM"),
                    &_ => println!("Unhandled vm command")
                }
                
            },
            "image" => println!("Command for image"),
            &_ => println!("Unhandled subcommand")
        }
        
    }
}

fn handle_command(command: Box<SubCommand>) {
    //let command_name = ;
    println!("Command: {}", command.name)
}