use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufRead};

struct Car {
    brand: String,
    year: u32,
}

fn reading_from_console() -> Car {
    let mut buffer = String::new();

    print!("What's your car brand? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let brand = buffer.trim().to_string();
    buffer.clear();

    print!("What year is your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    Car { brand, year }
}

fn append_to_file(car: &Car) {
    // opens file to append
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("user_info.txt")
        .unwrap();

    // write to file
    writeln!(file, "Brand: {}, Year: {}", car.brand, car.year).unwrap();
}

fn read_entire_file() {
    let mut file = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("\nFile contents:\n{}", contents);
}

fn read_file_line_by_line() {
    let file = File::open("user_info.txt").unwrap();
    let reader = BufReader::new(file);

    println!("\nReading file line by line:");
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    // create the struct
    let my_car = reading_from_console();

    // save car details to the file
    append_to_file(&my_car);

    // read and print the entire file content
    read_entire_file();
    read_file_line_by_line();
}