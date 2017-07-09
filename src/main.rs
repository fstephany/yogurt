use std::env;
use std::path::*;


enum Bitness {
    B32,
    B64
}

struct Vm {
    name: String,
    bitness: Bitness,
}

fn get_yogurt_workspace_path() ->  Option<PathBuf> {
    match env::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(".yogurt");
            Some(home_dir) 
        },
        None => None
    }
}

fn get_available_vms() -> Vec<Vm> {
    let mut vms = Vec::new();
    vms.push(Vm{ name: String::from("20170708"), bitness: Bitness::B64 });

    return vms
}

fn main() {
    match get_yogurt_workspace_path() {
        Some(path) => println!("Yogurt storage: {:?}", path),
        None => panic!("Could not find Yogurt working directory. Abort    ")
    }

    println!("Available VMs:");
    for vm in get_available_vms() {
        println!("- {}", vm.name);
    }
}
