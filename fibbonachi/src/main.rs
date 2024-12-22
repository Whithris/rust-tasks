use std::io;

fn main() {
    println!("Input n: ");
    let mut n = String::new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    println!("{}", fib(n))

}

fn fib(n: i32) -> i32 {
    if n == 1 || n == 2{
        1
    } else{
        fib(n-1)+fib(n-2)
    }
}