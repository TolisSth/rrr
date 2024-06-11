// Author: Apostolos Chalis 2024 <achalis@csd.auth.gr> 
// Time
use std::thread::sleep; 
use std::time::Duration; 

// File 
use std::fs; 
use std::fs::OpenOptions; 
use std::io::prelude::*;
use std::fs::File; 
use std::path::Path; 

// First party
mod daemonize;

fn is_rapl_mod_loaded() -> bool{
    let proc_modules = fs::read_to_string("/proc/modules").expect("ERROR: No /proc/modules found"); 

    if proc_modules.contains("intel_rapl_common") || proc_modules.contains("intel_rapl_msr"){
        return true; 
    }

    return false; 
}

fn does_log_file_exists(){
    let is_present = Path::new("/var/log/rrr.log").exists();    
    if is_present == false {let _create_log_file = File::create("/var/log/rrr.log").expect("ERROR: Unable to open file");}
}

fn main() {
    println!("Rust RAPL Reader v1.0\nAuthor: Apostolos Chalis <achalis@csd.auth.gr>\n----------------------------------------------"); 
    
    if is_rapl_mod_loaded() == true{
        println!("RAPL module found!")
    }
    else {
        println!("ERROR: No rapl module was loaded\nPlease load rapl module and try again.");
        std::process::exit(0); 
    }
    
    // Check if log file exists if doesn't create it, made this to cut off the check from the loop
    does_log_file_exists();
    println!("Created log file...");
    
    let path_to_energy_uj = Path::new("/sys/class/powercap/intel-rapl:0/energy_uj");
    let display_path = path_to_energy_uj.display(); 

    println!("Starting daemon...");
    daemonize::daemonize(); 

    // Daemon loop
    loop{
        // Opening file in Read only mode.
        let mut file = match File::open(&path_to_energy_uj) {
            Err(why) => panic!("couldn't open {}: {}", display_path, why),
            Ok(file) => file,
        };

        // Getting data from the file.
        let mut energy_uj_content = String::new();
            match file.read_to_string(&mut energy_uj_content) {
            Err(why) => panic!("couldn't read {}: {}", display_path, why),
            Ok(_) => {},
        }

        // Wait 500ms
        sleep(Duration::from_millis(500)); 

        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("/var/log/rrr.log")
            .expect("ERROR: Could not open rrr.log");

        log_file.write_all(energy_uj_content.as_bytes())
            .expect("ERROR: Could not open log file.");
    }
}
