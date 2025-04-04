fn main() {
    println!("please enter a natural number betwwen 1 and 10: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading input");
    let n: usize = input.trim().parse().expect("Please enter a natural number");
    if n>=1 && n<=10{println!("W{}w!", "o".repeat(n));
} else {println!(" i told you enter a natural number between 1 and 10 dummy");
}
}