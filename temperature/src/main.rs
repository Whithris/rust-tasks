use std::io;

fn main() {
    println!("Input current scale (F or C) and degree in different lines: ");
    let mut scale = String::new();
    
    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line");
    let scale: char = scale.trim().parse().expect("Please type a string!");

    let mut degree = String::new();
    io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read line");

    let degree: f32 = degree.trim().parse().expect("Please type a number!");

    if scale == 'F' {
        println!("In C it is {}", 0.555556*(degree - 32.0))
    } else if scale == 'C' {
        println!("In F it is {}", 1.8*degree + 32.0)
    }
}