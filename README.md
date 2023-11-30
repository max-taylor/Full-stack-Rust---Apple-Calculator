# Apple Calculator in Rust

This repository contains the source code for a desktop application that replicates the functionality and aesthetic of Apple's Calculator. It is built using Rust with Tauri and Yew frameworks, and styled using Tailwind CSS.

![Demo Video](https://github.com/max-taylor/Full-stack-Rust---Apple-Calculator/blob/main/example/demo_video.gif)

## Getting Started

These instructions will get you a copy of the project up and running on your local machine.

### Prerequisites

- Rust: [Installation Guide](https://www.rust-lang.org/tools/install)
- Tauri: [Setup Instructions](https://tauri.app/v1/guides/getting-started/prerequisites)
- Yarn: [Installation Guide](https://classic.yarnpkg.com/lang/en/docs/install/)
- npx: [Installation Guide](https://www.npmjs.com/package/npx)

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/max-taylor/Full-stack-Rust---Apple-Calculator.git
   cd apple-calculator-clone-rust
   ```

2. **Run the Development Server**

   ```bash
   cargo tauri dev
   ```

3. **Run the Tailwind Watcher service in a separate terminal**

   ```bash
   npx tailwindcss -i ./src/input.css -o ./output.css --watch
   ```

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used.
- [Tauri](https://tauri.studio/) - Framework for building lightweight, cross-platform desktop applications with web technologies.
- [Yew](https://yew.rs/) - Rust framework for building front-end web apps.
- [Tailwind CSS](https://tailwindcss.com/) - A utility-first CSS framework for rapidly building custom designs.

## License

This project is licensed under the MIT License.
