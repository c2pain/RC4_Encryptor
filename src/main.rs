use rc4::{Rc4, KeyInit, StreamCipher};
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args);
        return;
    }

    let input_filename = &args[1];

    let mut buf = read_file(&input_filename);
    let mut rc4 = Rc4::new(b"C2Pain".into());

    rc4.apply_keystream(&mut buf);

    // Extract the filename without the extension
    let filename_without_extension = match input_filename.rfind('.') {
        Some(index) => &input_filename[..index],
        None => input_filename,
    };

    let modified_filename: String = filename_without_extension.chars().map(|c| format!("{}-", c)).collect();

    let output_filename = format!("{}4.enc", modified_filename);
    write_file(&output_filename, &buf);

    println!("[+] Encrypted shellcode saved to: {}", output_filename);
}

fn print_usage(args: &[String]) {
    let path = args[0].clone();
    println!("[!] Usage: {} <input_filename_to_encrypt>", path);
    println!("[!] Example: {} Seatbelt.exe", path);
}

fn read_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Failed to read file");
    contents
}

fn write_file(filename: &str, content: &[u8]) {
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(content).expect("Failed to write to file");
}