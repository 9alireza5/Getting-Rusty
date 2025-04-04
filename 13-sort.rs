use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter two numbers separated by a space:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input.trim().split_whitespace();
    
    let mut x: i32 = numbers.next().expect("Missing first number").parse().expect("Invalid first number");
    let mut y: i32 = numbers.next().expect("Missing second number").parse().expect("Invalid second number");

    if x <= y {
        println!("Numbers are already sorted: x = {}, y = {}", x, y);
    } else {
        sort(&mut x, &mut y);
        println!("Sorted: x = {}, y = {}", x, y);
    }
}

fn sort(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
