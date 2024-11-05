# Commands

```
██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝
```

Commands is a simple command-line tool that lists and searches available Linux
commands directly in your terminal. It's designed to help users quickly discover
and explore the commands they can use in their Linux environment.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Makefile](#makefile)
- [Customization](#customization)
- [Contributing](#contributing)
- [License](#license)

## Features

- Displays a comprehensive list of all available Linux commands
- Simple and intuitive command-line interface
- Supports searching for specific commands
- Gradual display of commands for better readability

## Installation

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

### Building from Source

1. Clone the repository:

```
git clone https://github.com/juanmilkah/linux-command-search.git
```

2. Navigate to the project directory:

```
cd linux-command-search
```

3. Use the Makefile to build and install the project (see [Makefile](#makefile) section for more details):

```
make install
```

4. Follow the instructions provided after installation to update your PATH.

## Usage

To use the Commands tool:

1. List all available commands:

```bash
commands
```

or

```bash
commands -l
```

or

```bash
commands --list
```

2. Search for a specific command:

```bash
commands -s <search_term>
```

or

```bash
commands --search <search_term>
```

3. Display help information:
```bash
commands -h
```
or
```bash
commands --help
```

### Example Output

```bash
Available Linux Commands:
1. ls - List directory contents
2. cd - Change directory
3. cp - Copy files and directories
...
```

## Makefile

The project includes a Makefile to simplify the build and installation process. Here are the available commands:

- `make build`: Compiles the project
- `make install`: Installs the project to ~/commands and provides instructions for updating PATH
- `make clean`: Removes build files
- `make help`: Shows the help message with available commands

To install the project, run:

```
make install
```

After installation, follow the instructions provided to add the installation directory to your PATH. You'll need to add a line to your shell configuration file (.bashrc, .bash_profile, or .zshrc) and then either source the file or restart your terminal for the changes to take effect.

## Customization

You can easily add or modify commands by editing the `linux` file in the project root. Each line should follow this format:

```
[command_name]  Command description
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/AmazingFeature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
5. Push to the branch (`git push origin feature/AmazingFeature`)
6. Open a Pull Request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
