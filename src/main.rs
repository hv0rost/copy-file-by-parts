use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let first_file_path = String::from("C:/Users/Hv0rost/Desktop/ROOT/Office_Professional_Plus64Bit_RUS.zip");
    let second_file_path = String::from("C:/Users/Hv0rost/Desktop/Office_Professional_Plus64Bit_RUS.zip");
    let bytes_to_read : u64 = 512;

    let metadata = fs::metadata(&first_file_path).unwrap();

    let mut rest_of_file = metadata.len();
    let mut buf = vec![0u8; bytes_to_read as usize];

    File::create(&second_file_path).unwrap();

    let mut first_file = File::open(first_file_path).unwrap();

    let mut second_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(second_file_path)
        .unwrap();


    while rest_of_file > bytes_to_read {
        println!("Left bytes for copy: {}", rest_of_file);
        first_file.read_exact(&mut buf).unwrap();
        second_file.write(&mut buf).unwrap();

        rest_of_file = rest_of_file - bytes_to_read;
    }

    let mut buf = vec![0u8; rest_of_file as usize];

    println!("Left bytes for copy: {}", rest_of_file);
    first_file.read_exact(&mut buf).unwrap();
    second_file.write(&mut buf).unwrap();
}
