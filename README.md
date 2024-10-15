# Cybersecurity Tools

This repository contains a collection of cybersecurity tools developed in Rust.

## Tools

### 1. FTP Worm Simulation

*   **Description:** This tool simulates an FTP worm to demonstrate potential security risks associated with insecure FTP servers.
*   **Features:**
    *   Connects to an FTP server and authenticates with user-provided credentials.
    *   Checks if the server is unsecured by attempting to create a directory and upload a file.
    *   Simulates worm propagation by recursively listing directories and copying the current executable to those directories.
    *   Simulates payload delivery by copying existing files with multiple iterations.
*   **Usage:**
    ```bash
    cargo run -- worm -t <target_ip> 
    ```

### 2. Folder Encryption Tool

*   **Description:** This tool encrypts and decrypts folders to protect sensitive data.
*   **Features:**
    *   Encrypts or decrypts files and folders using a strong encryption algorithm (e.g., AES-256).
    *   Password-based encryption and decryption.
    *   Recursive folder encryption and decryption.
    *   Multi-threaded operation for efficient processing of large folders.
*   **Usage:**
    ```bash
    cargo run -- encrypt -f <folder_path> -p <password>
    cargo run -- decrypt -f <folder_path> -p <password>
    ```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests for bug fixes, improvements, or new   
 tool suggestions.
