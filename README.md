(Not really a) process management application

This project is a simple process management application written in Rust. But it's not a real application. You should regard this code as a kind of slightly more elaborate “Hello World” designed by a beginner in Rust programming.
It's designed as a didactic exercise to demonstrate various aspects of programming in Rust. It's not a fully functional application (although it works perfectly), but rather a teaching tool.


## Features

- Display processes sorted by CPU usage
- Display processes sorted by memory usage
- Stop a process by its number in the list
- User interaction through terminal input

## Rust Concepts Covered

### 1. External Crates

The project uses the `sysinfo` crate to gather information about system processes. External crates are managed using Cargo, Rust's package manager.

```toml
[dependencies]
sysinfo = "0.21"
```

### 2. Structs and Methods

The `sysinfo` crate provides various structs and methods to interact with system information. For example, `System`, `Process`, and their associated methods like `refresh_all`, `processes`, `cpu_usage`, and `memory`.

### 3. Input/Output

The program uses standard input/output to interact with the user. The `std::io` module is used to read user input and print messages to the terminal.

```rust
use std::io::{self, Write};
```

### 4. Control Flow

The program demonstrates basic control flow constructs such as loops, match statements, and conditional statements.

```rust
loop {
    display_welcome_message();
    if !wait_for_user_input() {
        break;
    }
}
```

### 5. Error Handling

The program includes basic error handling using `Result` and `Option` types. For example, reading user input and parsing it to a number.

```rust
match num_processes.trim().parse() {
    Ok(num) => Some(num),
    Err(_) => {
        println!("Please enter a valid number");
        None
    }
}
```

### 6. Multithreading

The program uses the `std::thread` module to pause execution for a short duration to allow CPU statistics to update.

```rust
thread::sleep(Duration::from_secs(1));
```

### 7. System Commands

The program demonstrates how to execute system commands using the `std::process::Command` module to clear the terminal.

```rust
Command::new("clear").status().unwrap();
```

## How to Run

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository.
3. Navigate to the project directory.
4. Run the application using Cargo:

```sh
cargo run
```

## Conclusion

This project is a simple exercise to demonstrate various Rust programming concepts. It is not intended to be a fully functional application but rather an educational tool to help you learn Rust. Feel free to explore the code, modify it, and experiment with different features to deepen your understanding of Rust.

Have fun, Rust in Peace ! 
MSC
