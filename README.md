Minigrep

A miniature implementation of the Unix grep utility built while working through The Rust Programming Language.

Overview

This project searches for text within files from the command line. It is intended as a learning project to practice core Rust concepts, including:

Command-line argument parsing
File I/O
Error handling
Ownership and borrowing
Modules and project organization
Testing
Getting Started

Clone the repository and run the project with Cargo:

cargo run -- <query> <file>

Example:

cargo run -- hello poem.txt