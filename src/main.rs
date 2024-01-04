use sysinfo::{
    System,
};



use std::thread;
use std::time::Duration;

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        println!("-----------------PROCESSOR------------------");
        for cpu in system.cpus() {
            println!("{} {}%", cpu.name(), (cpu.cpu_usage() * 100.0).round() / 100.0);
        }
        println!("--------------------------------------------");

        println!(" ");

        println!("-------------------MEMORY--------------------");
        println!(
            "Memory Usage: {} MB / {} MB",
            system.used_memory() / 1024,
            system.total_memory() / 1024
        );
        println!("--------------------------------------------");

        thread::sleep(Duration::from_secs(5));
    }
}
