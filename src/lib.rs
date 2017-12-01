extern crate reqwest;

use std::path::PathBuf;
use std::io::Read;
use std::fs;

enum Platform {
    MacOS,
    Linux
}

struct Vm {
    name: String,
    platform: Platform
}

/// Contains the necessary config information to fetch and store VMs and images
struct Config {
    platform: Platform,
    base_url: String,
    local_storage_dir: PathBuf
}

/// Main data structure of Yogurt.
struct Yogurt {
    config: Config
}

impl Yogurt {

    // Construction

    fn new(config: Config) -> Yogurt {
        Yogurt { config: config }
    }

    // Accessing

    fn vms_dir(&self) -> PathBuf {
        self.config.local_storage_dir.join("vms")
    }

    fn images_dir(&self) -> PathBuf {
        self.config.local_storage_dir.join("images")
    }

    fn sources_dir(&self) -> PathBuf {
        self.config.local_storage_dir.join("sources")
    }

    fn arch_dirname(&self) -> String {
        match self.config.platform {
            Platform::MacOS => String::from("vms-macos-x86-64"),
            Platform::Linux => String::from("vms-linux-x86-64")
        }
    }

    fn vms_remote_url(&self) -> String {
        format!("{}/{}/list.txt", self.config.base_url, self.arch_dirname())
    }

    // Actions

    pub fn create_dirs_if_necessary(&self) {
        fs::create_dir_all(self.vms_dir()).expect("Could not create VMs directory. Abort");
        fs::create_dir_all(self.images_dir()).expect("Could not create images directory. Abort");
        fs::create_dir_all(self.sources_dir()).expect("Could not create sources directory. Abort");
    }

    /// Returns all the VMs stored on disk.
    pub fn available_vms_on_disk(&self) -> Vec<Vm> {
        let mut vms = Vec::new();
        let dirs_to_scan = fs::read_dir(self.vms_dir()).unwrap();

        for entry in dirs_to_scan {
            let path = entry.unwrap().path();
            let dir_name = path.file_name().unwrap().to_str().unwrap();

            if path.is_dir() {
                vms.push(Vm{ name: String::from(dir_name), platform: self.config.platform });
            }
        }

        return vms
    }

    /// List all available VMs on remote server
    fn available_vms_on_remote(&self) -> Vec<Vm> {
        let url = self.vms_remote_url();
        let mut resp = reqwest::get(&url)
            .expect(&format!("Could not fetch VMs list from {}", url));

        if !resp.status().is_success() {
            panic!("Could not fetch VMs List from {}.\nHTTP Status: {}", url, resp.status())
        }

        let mut content = String::new();
        resp.read_to_string(&mut content);

        return content
            .lines()
            .map(|line| Vm{ name: String::from(line), platform: self.config.platform})
            .collect();
    }

    /// Fetches all the VMs stored on the server for a given architecture
    fn fetch_vms(&self) -> Vec<Vm> {
        return Vec::new();
    }
}