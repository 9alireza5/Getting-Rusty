fn main() {
    println!("please enter the row and column, separated by a space: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut inputs = input.trim().split_whitespace();
    let row:u8=inputs.next().expect("enter row, and then column with a space between them").parse().expect("Please enter a valid number for row");
    let column:u8=inputs.next().expect("enter column as well").parse().expect("Please enter a valid number for column");
    //println!("row is {} and column is {}", row, column);
    if column >= 1 && column <= 10 {
        println!("Right {} {}", 11-row, column);
    } else if column >= 11 && column <= 20 {
        println!("Left {} {}", 11-row, 21 - column);
    } else {
        println!("Invalid seat number. Please enter a column between 1 and 20.");
    }
}