use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), std::io::Error> {
 
    // открываем файл example.txt
    let mut file = File::open("input.txt")?;

    let mut content = String::new();

    let reader = BufReader::new(file); 

    for i in reader.lines(){
        let line = i?;
        println!("{}", line);
    }
    file.read_to_string(&mut content)?;
 
    println!("{}", content);
    Ok(())
}