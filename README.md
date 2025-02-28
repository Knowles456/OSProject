# Rust Inter-Process Communication (IPC) and Deadlock Examples

## Overview

This project demonstrates two key concepts in Rust programming: creation and management of threads that demonstrate principles of concurrency and
thread safety, mutex locks in controlling access to shared resources properly and multi-threaded process deadlock detection, resolution and avoidance.

1.  **Multi-Threading Implementation** Showcases concurrent thread operations through spawning multiple threads, with each thread given the purpose of adding an "ingredient" to a recipe and doing so at roughly the same time as each other.

2.  **Deadlock Scenarios and Resolution:** demonstrating an example of dead-lock, detecting that deadlock, and resolving it. My solution involved creating two separateshared resources that will be accessed and written to by two threads at a time. The solution is formatted into sections, one which provides deadlock through a circular wait condition and detects it, and the other which resolves and avoids deadlock by proper sequencing and a timeout function. The first section mandates deadlock through both structuring the threads to access their shared resources in reverse order and through sleep calls. The order in which deadlock occurs is as follows: thread 1 locks the vector "recipe", thread two locks the vector "extra ingredients", thread one adds a string to recipe, thread 2 does the same to extra in- gredients, thread 1 attempts to get the extra ingredients lock but cannot because thread 2 is still holding it, and thread 2 encounters the same problem when attempting to access recipe

3.  **Inter-Process Communication (IPC) using Pipes:**  This section showcases how to create two separate Rust programs that communicate with each other using operating system pipes.  We implement a simple producer-consumer pattern where one program generates numbers and sends them through a pipe to another program that squares the numbers.

## Dependencies and Installation

**Dependencies:**

 *   **Rust Toolchain:** This project is written in Rust and requires the Rust toolchain to be installed.

**Installation Instructions:**

1.  **Install Rust:** Follow the official Rust installation instructions for your operating system: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
    *   On Linux/macOS, you typically use `rustup`in the terminal.
    *   On Windows, download and run the `rustup-init.exe` installer from the link above.

2.  **Verify Installation:** After installation, open a terminal and check if Rust is installed correctly:
    rustc --version
    cargo --version
    You should see version numbers for `rustc` (the Rust compiler) and `cargo` (the Rust build system and package manager).

**Setting up WSL (Windows Subsystem for Linux)**

It's required for all sections of the project to setup a linux environment in which to execute these programs. Functionality realted to IPC and thread management is predicated on having the linux toolkit within scope of these executables.

1.  **Enable WSL:**
    *   Open PowerShell as Administrator and run: wsl --install
    *   Follow the on-screen instructions to complete the installation. You'll want to select y to all questions.

2.  **Choose a Linux Distribution (if prompted):** You may be asked to decide on a Linux distribution. I chose Ubuntu.

3.  **Access WSL:** Search for WSL in the windows start bar and select it. It will open a terminal that you can then install requisite packages from to your new Linux environment

4.  **Install Rust in WSL (if not already installed in Windows and you want to use WSL for development):** You can install Rust inside your WSL Linux distribution following the same  using `rustup`. This is as mentioned before in the install rust section.

## Building and Running the Programs

**Building:**
1.  **Locating the project files** Considering you're reading this readme file, that means you've already located the Github for this project. There are three different programs (as listed earlier) you can now choose to compile and execute. Within Github, select the branch that corresponds with what you're looking for whether that demonstrating basic multi-threaded actions (phase1), deadlock and deadlock resolution(phase3), or IPC(ProjectB).

2.  **Navigate to the project directory:** Open a terminal type cd hello_rust/OSProject/food_exchange/src and hit enter

3.  **Basic Multithreaded Demonstration:**
        cargo build
        This will compile the `main.rs` file (containing the deadlock code) and create an executable in the `target/debug` directory (e.g., `target/debug/ipc_project` or `target/debug/ipc_project.exe`).

4.  **Compile IPC:**
    *   Compile `generate_numbers.rs` and `square_numbers.rs` using `rustc`:
        This will create executable files named `generate_numbers` and `square_numbers` (or `.exe` on Windows) in the `src` directory.

5.  **Compile Deadlock:**
        cargo build
        This will compile the `main.rs` file (containing the deadlock code) and create an executable in the `target/debug` directory (e.g., `target/debug/ipc_project` or `target/debug/ipc_project.exe`).

**Running:**

**1. Inter-Process Communication (IPC) Example:**

*   **Run with a pipe:** Execute the following command in your terminal to run `generate_numbers` and pipe its output to `square_numbers`:
    ./generate_numbers | ./square_numbers

    You should see the squared numbers (1, 4, 9, 16, 25) printed to the terminal.

**2. Deadlock and Basic Multithreaded Demonstration:**

    Ensure you're in the project root directory in your terminal.
*   input ./main into the terminal

