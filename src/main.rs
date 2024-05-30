// Author: Apostolos Chalis 2024 <achalis@csd.auth.gr> 
use std::fs; 

fn is_rapl_mod_loaded() -> bool{
    let proc_modules = fs::read_to_string("/proc/modules").expect("ERROR: No /proc/modules found"); 

    if proc_modules.contains("intel_rapl_common") || proc_modules.contains("intel_rapl_msr"){
        return true; 
    }

    return false; 
}

fn main() {
    println!("Rust RAPL Reader v1.0\nAuthor: Apostolos Chalis <achalis@csd.auth.gr");
    
    if is_rapl_mod_loaded() == true{
        println!("OK\n");
        // daemonize(); 
    }
    else {
        println!("ERROR: No rapl module was loaded\nPlease load rapl module and try again.");
    }
}
