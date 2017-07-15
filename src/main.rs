use std::env;
use std::fs;
use std::path::*;


enum Bitness {
    B32,
    B64
}

struct Vm {
    name: String,
    bitness: Bitness,
}

fn yogurt_workspace_path() ->  Option<PathBuf> {
    match env::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(".yogurt");
            Some(home_dir) 
        },
        None => None
    }
}

fn vms_dir() -> PathBuf {
    yogurt_workspace_path()
        .expect("Could not find workspace directory")
        .join("vms")
}

fn images_dir() -> PathBuf {
    yogurt_workspace_path()
        .expect("Could not find workspace directory")
        .join("images")
}

fn sources_dir() -> PathBuf {
    yogurt_workspace_path()
        .expect("Could not find workspace directory")
        .join("sources")
}

fn create_dirs_if_necessary() {
    fs::create_dir_all(vms_dir()).expect("Could not create VMs directory. Abort");
    fs::create_dir_all(images_dir()).expect("Could not create images directory. Abort");
    fs::create_dir_all(sources_dir()).expect("Could not create sources directory. Abort");
}

fn available_vms() -> Vec<Vm> {
    let mut vms = Vec::new();
    let dirs_to_scan = fs::read_dir(vms_dir()).unwrap();

    for entry in dirs_to_scan {
        let path = entry.unwrap().path();
        let dir_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            vms.push(Vm{ name: String::from(dir_name), bitness: Bitness::B64});
        }
    }

    return vms
}

fn print_available_vms() {
    println!("Available VMs:");
    for vm in available_vms() {
        println!("- {}", vm.name);
    }
}

fn main() {
    create_dirs_if_necessary();
    print_available_vms();
}
