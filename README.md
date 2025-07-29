# RCrab--1-
#  Rust CLI Tools: File Sorter & HTTP Status Code Explainer

A collection of two beginner-friendly CLI tools written in Rust, built under a single Cargo project using Rust's `src/bin/` structure.

---

##  Project 1: File Sorter

Organizes files in the current directory based on file extensions.

###  Features

- Moves `.jpg`, `.png` → `images/`
- Moves `.mp4`, `.mkv` → `videos/`
- Moves `.txt`, `.md` → `docs/`
- Other file types → `others/` or ignored

### Rust Concepts

- [`std::fs`]("https://doc.rust-lang.org/std/fs/ " ) for file manipulation
- Pattern matching on file extensions
- Error handling with `Result` and `?`

### Run It

```bash
cargo run --bin file_sorter
```
-- 
## Project 2: HTTP Status Code Explainer
### Explains HTTP status codes passed as command-line arguments.

### Features
- Input: numeric status code `(e.g. 404)`

- Output: string explanation `(Not Found)`

### Rust Concepts
- CLI input via [`std::env::arg `] ("https://doc.rust-lang.org/beta/std/env/fn.args.html")

- enum to represent known status codes

- Pattern matching & modular design

### Run It
```bash 
cargo run --bin http_explainer 404
```
