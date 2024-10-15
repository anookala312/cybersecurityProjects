mod lib;
#[macro_use]
extern crate arrayref;
    
use rpassword::read_password;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::thread;
use std::io;
use std::fs;
use std::env;
use std::path::PathBuf;
use std::thread::JoinHandle;
use std::thread::sleep;
use std::time;
use colored::Colorize;

fn arguments() -> (String, bool, bool, [u8;32]) {
    // Setting the Usage for directories
    let args: Vec<String> = env::args().collect();


    let size = &args.len().to_owned();

    if *size as i32 == 4  {
        let buf = args.get(1).unwrap();
        // Selecting and Converting String to a Path object 

        let path = PathBuf::from(buf);
        if !path.exists() {
            println!("Path dosn't exists");
            exit(1)
        }

        let sub:bool = args.get(2).unwrap().parse::<i32>().unwrap() != 0;
        let enc:bool = args.get(3).unwrap().parse::<i32>().unwrap() != 0;
        let _password = lib::password_to_key(args.get(4).unwrap().to_string());
        return (path.to_string_lossy().to_string(), sub, enc,_password)
    }

    let mut buf = String::new();
    let path: PathBuf;
    let mut sub_d:bool = false;
    let  enc;
    let mut _password: String;


    println!("Path: ");

    let _handle = io::stdin();

    // Reading the path
    _ = _handle.read_line(&mut buf);
    println!("{:?}",buf.replace("\n", ""));
    // Converting String to Path Object 
    path = PathBuf::from(buf.replace("\n", ""));
    if !path.exists() {
        println!("Path dosn't exists");
        exit(1)
    }
    let mut buf = String::new();

    if path.is_dir() {
        println!("Also subdirs? [y/n]: ");

        _ = _handle.read_line(&mut buf);
        
        if buf.replace("\n", "").contains("y") {
           sub_d = true; 
        } else {sub_d = false;}
    }

    //Selecting the Options -- To Encrypt (E) or Decrypt (D)
    println!("Do You want the file/s {} or {}", "ENCRYPTED [E]".bold(),"DECRYPTED [D]".bold());
    println!("Choose Operation? [E/D] 'E - Encrypt/ D - Decrypt': ");
    buf.clear();
    _ = _handle.read_line(&mut buf);
    if buf.contains("E") || buf.contains("e") {
        println!("{} was selected","ENCRYPTION".bold().green());
        enc = true;
    } else {
        println!("{} was selected","Decryption".bold().green());
        enc = false;
    }

    //Enter Password To either Decrypt or Encrypt
    println!("Enter a Password: ");


    std::io::stdout().flush().unwrap();
    let _password = lib::password_to_key(read_password().unwrap());

    return (path.to_string_lossy().to_string(),sub_d,enc,_password);
    
}


fn main() {
    
    let r = arguments();
    let op;
    if r.2 {
        op = "encrypting".green();
    } else {
        op = "decrypting".green();
    }
    println!("{:#?}",r);

    //After reading directory, deletes all sub-dirs
    println!("Now {op} {:#?},\ndeletion of all its sub-dirs: {:#?}\n{}",r.0,r.1,"Press Enter to continue.".yellow());
    
    let mut input = String::new();

    // Waiting for User Input
    let handle = io::stdin();    
    handle.read_line(&mut input).expect("ERROR OCCURED, EXITING NOW");
    
    // Pause
    let now = std::time::Instant::now();
    
    if PathBuf::from(&r.0).is_dir() {    
        // Read all the files in dir
        let re = get_files(&r.0,r.1);
        println!("{:#?}",re);

        interact_with_files(re.clone().0.clone(), r.2, r.3);
        println!("Found {:#?} files",re.1);

    } else {
        interact_with_files(vec![PathBuf::from(&r.0)], r.2, r.3);
        println!("Operation succeded on: {}", r.0.green());

    }

    // Stop Pause
    let took = now.elapsed();

    // Summary of time taken to encrypt/ decrypt
    println!("It took : {:#?}!",took)
    

}

fn interact(path: PathBuf,encrypt: bool, n: u8, key: [u8;32]) {
    if encrypt {
        let nonce = lib::rand_key_nonce().1;

        lib::encrypt_data_file(&path.to_string_lossy().to_string(), &key, &nonce).expect("Couldn't encrypt! -\\");

        println!("[{n}] Encrypted: {:#?}",path);

    } else {
        if path.to_string_lossy().to_string().contains(".enc")  {
            let mut f = fs::File::open(&path).expect("Couldn't Decrypted Data");
            
            let mut buf = vec![0; 19];

            f.read_exact(&mut buf).expect("Couldn't Decrypted Data");
            lib::decrypt_data_file(&path.to_string_lossy().to_string(), &key, array_ref![buf,0,19]).expect("Couldn't decrypt_data! -\\");
            println!("[{n}] Decrypted: {:#?}",path);
            

        }   else {
            println!("not")
        }
    }
    fs::remove_file(&path).expect(&"Couldn't delete inital file!".red());

}

fn get_files(path: &String, sub: bool) -> (Vec<PathBuf>, i32) {
    let mut count: i32 = 0;
    let mut _files: Vec<PathBuf> = vec![]; 
    for a in fs::read_dir(path).unwrap() {
        let a_path = a.unwrap().path();
     
        if a_path.is_dir() && sub {
            let path_files = get_files(&a_path.to_string_lossy().to_string(),sub);
            for b in path_files.0 {
                _files.push(b);

            }
            count = count + path_files.1
        }
        
        count = count + 1;
        if a_path.is_file() {
            _files.push(a_path)
        }
    }
    return (_files,count);
}

fn thread_run(list: Vec<PathBuf>,encrypt: bool,n: u8, key: [u8;32]) -> JoinHandle<()> {
    let a = thread::spawn(move|| {
        for file in list {
            interact(file, encrypt, n, key);

        }
        println!("[{n}] {} ","FINISHED".green());        
    });
    return a;
}

static NUMBER_OF_THREADS: usize = 4;

fn interact_with_files(files_:Vec<PathBuf>,encrypt:bool, key: [u8;32]) {    
    let mut threads: Vec<JoinHandle<()>> = vec![];
    let mut n: u8 = 0;
    let file_iter;
    if files_.len() as u8 <= 8 {
        file_iter = files_.chunks(files_.len()).into_iter()
    } else {
        file_iter = files_.chunks(files_.len()/NUMBER_OF_THREADS).into_iter()
    }


    for i in file_iter {
        n = n +1;
        threads.push(thread_run(i.to_vec(), encrypt,n, key));
    }
    let mut b = false;
    loop {

        sleep(time::Duration::from_millis(500));
        if b {break;}
        let mut a = 0;
        for t in &threads {
            if t.is_finished() {
                b = true;
                
            } else {
                b = false;
            }
            a = a +1;
        }
    }
    println!("{}","PROCESS COMPLETED SUCCESSFULLY!".green());
}