# 📊 System Monitor in Rust

This is a command-line application written in Rust that uses the [`sysinfo`](https://docs.rs/sysinfo/) crate to display detailed system information in real time. It includes memory usage, disk details, network activity, and component temperatures.

## 🚀 Features

- Display system name, kernel version, OS version, and host name  
- Show total and used memory, as well as swap usage  
- List connected disks with detailed information  
- Display network traffic (received and transmitted data)  
- Show temperature readings of system components (CPU, GPU, etc.)  
- Interactive menu for selecting the desired information  

## 🛠️ How to Use

1. Make sure [Rust](https://www.rust-lang.org/tools/install) is installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/Clebson-Torres/System-Monitor-in-Rust.git
   cd System-Monitor-in-Rust
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## 📦 Dependencies

- [`sysinfo`](https://crates.io/crates/sysinfo)

This dependency is automatically handled by Cargo when you build the project.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
