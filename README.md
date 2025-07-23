# AI Commit Tool

## Overview
The AI Commit Tool is a Git hook utility designed to assist developers in crafting meaningful commit messages using AI. It integrates seamlessly with Git, providing hooks that enhance the commit process by suggesting and formatting commit messages based on the changes made.

## Features
- **AI-Assisted Commit Messages**: Automatically generates commit message suggestions based on the changes detected in the repository.
- **Customizable Hooks**: Provides hooks for modifying commit messages before they are finalized and executing actions after a commit is made.
- **Easy Installation and Uninstallation**: Simple commands to install or uninstall the Git hooks as needed.

## Installation
To install the AI Commit Tool, run the following command in your terminal:

```bash
cargo run --bin install
```

This command sets up the necessary Git hooks in your repository.

## Usage
Once installed, the AI Commit Tool will automatically suggest commit messages when you make changes to your repository. You can customize the behavior by modifying the templates located in the `templates` directory.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.