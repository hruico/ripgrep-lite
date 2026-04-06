# 🔍 ripgrep-lite

A high-performance command-line search utility built from the ground up in **Rust**. This project is a focused reimagining of the classic `ripgrep`, designed to explore the intersection of systems programming and efficient pattern matching.

## 🎯 Project Goal
The aim of `ripgrep-lite` is to dismantle the complexity of modern search tools. By building this "lite" version, I am exploring how Rust handles:
* **Memory-mapped file I/O** for rapid data access.
* **The Borrow Checker** to ensure thread-safe searching.
* **Zero-cost abstractions** to keep performance close to the metal.

## 🛠️ Current Features
- [x] **Recursive Directory Search:** Quickly traverses folders to find your strings.
- [x] **Case Sensitivity Toggles:** Options to respect or ignore casing.
- [ ] **Regex Support:** (Coming Soon) Leveraging specialized crates for pattern matching.
- [ ] **Multithreaded Scanning:** Parallelizing the search process for massive directories.

## 🚀 Getting Started

### Prerequisites
Ensure you have the [Rust toolchain](https://rustup.rs/) installed on your system.

### Installation
```bash
git clone [https://github.com/your-username/ripgrep-lite.git](https://github.com/your-username/ripgrep-lite.git)
cd ripgrep-lite
cargo build --release