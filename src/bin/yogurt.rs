extern crate clap;

extern crate yogurt;

use std::env;
use std::process::Command;
use std::error::Error;
use std::path::*;

use clap::{App, Arg, SubCommand};

use yogurt::*;


fn print_available_vms() {
    println!("Available VMs:");
    for vm in available_vms() {
        println!("- {}", vm.name);
    }
}

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
            path
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
            base_url: "https://localhost:4040/",
            local_storage_dir: get_dev_local_storage().expect("Could not find current_dir() for local storage")
        }
    } else {
        Config {
            base_url: "";
            local_storage_dir: get_regular_local_storage().expect("Could not find local storage")
        }
    };


    println!("DEV: {}", is_dev_env);

    create_dirs_if_necessary(config);
    print_available_vms();
    
    let remote_vms = match fetch_remote_vms() {
        Ok(vms) => vms,
        Err(err) => Vec::new()
    };

    for vm in remote_vms {
        println!("- {}", vm.name);
    }

    let version_string = "0.1.0";


    let matches = clap::App::new("Yogurt")
        .version(version_string)
        .about("The Pharo toolchain installer")

        // VM commands
        .subcommand(SubCommand::with_name("vm")
            .about("VM related features")
            .subcommand(SubCommand::with_name("list")
                .about("list installed VMs"))

            .subcommand(SubCommand::with_name("install")
                .about("install a VM")
                .arg(Arg::with_name("version")
                    .help("The version to install"))))

        // Image commands
        .subcommand(SubCommand::with_name("image")
            .about("Image related features"))
        
        // Params processing
        .get_matches();

//    matches.

    // Command::new("/home/fstephany/.yogurt/vms/20170708/bin/pharo")
    //         .arg("blop")
    //         .spawn()
    //         .expect("Could not start pharo");
}
