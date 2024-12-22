use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("text.txt")?;
    let reader = BufReader::new(file); // Создаем буфер
    let mut strings: Vec<Vec<String>> = Vec::<Vec<String>>::new();
    // построчно считываем текст
    for line in reader.lines() {
        strings.push(line?.split(" ").map(|s| s.to_string()).collect());

    }
    let mut index = 0;
    if let Some(first_vec) = strings.get(0) {
        while index < first_vec.len() {
            for element in strings.iter() {
                    print!("{} ", element[index]);
                }
            println!("");
            index += 1;
            }
        }
    Ok(())
}