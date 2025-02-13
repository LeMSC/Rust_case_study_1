use sysinfo::{System, SystemExt, ProcessExt, Signal};
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Main entry point of the application.
fn main() {
    loop {
        display_welcome_message();
        if !wait_for_user_input() {
            break;
        }
    }
}

/// Displays the welcome message and available options.
fn display_welcome_message() {
    // Clear the terminal
    Command::new("clear").status().unwrap();

    println!("--------------------------------------------------------------------");
    println!("Welcome to the process management application!");
    println!("--------------------------------------------------------------------");
    println!("Choose an option:");
    println!("-----------------");
    println!("1. Display processes sorted by CPU usage");
    println!("2. Display processes sorted by memory usage (Default)");
    println!("3. Stop a process");
    println!("4. Quit the application");
    println!();
}

/// Waits for and processes user input.
fn wait_for_user_input() -> bool {
    print!("Please enter your choice: ");
    io::stdout().flush().unwrap(); // Ensure the message is displayed before reading input

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Error reading input");
        return true;
    }
    let input = input.trim(); // Clean the string

    match input {
        "1" => {
            if let Some(num_processes) = get_num_processes() {
                display_top_processes(num_processes, "cpu");
            }
        },
        "2" => {
            if let Some(num_processes) = get_num_processes() {
                display_top_processes(num_processes, "memory");
            }
        },
        "3" => {
            if let Some(num_processes) = get_num_processes() {
                let processes = display_top_processes(num_processes, "memory");

                print!("Please enter the process number to stop: ");
                io::stdout().flush().unwrap();

                let mut process_number = String::new();
                if io::stdin().read_line(&mut process_number).is_err() {
                    println!("Error reading input");
                    return true;
                }
                let process_number: usize = match process_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number");
                        return true;
                    }
                };

                stop_process(process_number, &processes);
            }
        },
        "4" => {
            println!("Quitting the application...");
            return false;
        },
        _ => println!("Invalid option"),
    }

    // Wait for the user to press Enter before returning to the main menu
    println!("Press Enter to continue...");
    let mut _pause = String::new();
    if io::stdin().read_line(&mut _pause).is_err() {
        println!("Error reading input");
    }

    true
}

/// Asks the user how many processes they want to display and returns that number.
fn get_num_processes() -> Option<usize> {
    print!("How many processes do you want to display? ");
    io::stdout().flush().unwrap();

    let mut num_processes = String::new();
    if io::stdin().read_line(&mut num_processes).is_err() {
        println!("Error reading input");
        return None;
    }
    match num_processes.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Please enter a valid number");
            None
        }
    }
}

/// Displays processes sorted by CPU or memory usage.
/// Returns the list of displayed processes.
fn display_top_processes(num_processes: usize, sort_by: &str) -> Vec<sysinfo::Process> {
    let mut system = System::new_all();
    system.refresh_all();

    if sort_by == "cpu" {
        // Wait a short moment to allow CPU statistics to update
        thread::sleep(Duration::from_secs(1));
        system.refresh_all();
    }

    let mut processes: Vec<_> = system.processes().values().cloned().collect();
    match sort_by {
        "cpu" => processes.sort_by_key(|p| -(p.cpu_usage() as i64)),
        "memory" => processes.sort_by_key(|p| -(p.memory() as i64)),
        _ => (),
    }

    println!("{:<10} {:<10} {:<60} {:<10}", "Number", "PID", "Name", "Usage");
    println!("{:<10} {:<10} {:<60} {:<10}", "======", "===", "====", "=====");
    for (i, process) in processes.iter().take(num_processes).enumerate() {
        match sort_by {
            "cpu" => println!("{:<10} {:<10} {:<60} {:<10.2} %", i + 1, process.pid(), process.name(), process.cpu_usage()),
            "memory" => println!("{:<10} {:<10} {:<60} {:<10} KB", i + 1, process.pid(), process.name(), process.memory()),
            _ => (),
        }
    }

    processes
}

/// Attempts to kill the process specified by its number in the process list.
fn stop_process(process_number: usize, processes: &[sysinfo::Process]) {
    if let Some(process) = processes.get(process_number - 1) {
        if process.kill(Signal::Kill) {
            println!("Process {} (PID: {}) has been stopped.", process_number, process.pid());
        } else {
            println!("Unable to stop process {} (PID: {}).", process_number, process.pid());
        }
    } else {
        println!("Invalid process number.");
    }
}