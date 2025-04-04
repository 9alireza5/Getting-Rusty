fn main() {
    println!("please enter angles: ");
    let mut angle1 = String::new();
    std::io::stdin().read_line(&mut angle1).expect("Failed to read input");
    let angle1: i32 = angle1.trim().parse().expect("Please enter a valid number");
    if angle1 < 0 || angle1 >= 360 {
        println!("bagher,please enter an intigier betwwen 0 and 360");
        return;
    }
    let mut angle2 = String::new();
    std::io::stdin().read_line(&mut angle2).expect("Failed to read input");
    let angle2: i32 = angle2.trim().parse().expect("Please enter a valid number");
    if angle2 < 0 || angle2 >= 360 {
        println!("bagher,please enter an intigier betwwen 0 and 360");
        return;
    }
    let mut angle3 = String::new();
    std::io::stdin().read_line(&mut angle3).expect("Failed to read input");
    let angle3: i32 = angle3.trim().parse().expect("Please enter a valid number");
    if angle3 < 0 || angle3 >= 360 {
        println!("bagher,please enter an intigier betwwen 0 and 360");
        return;
    }
    if angle1 + angle2 + angle3 == 180 && angle1 > 0 && angle2 > 0 && angle3 > 0 {
        println!("yes");
    } else {
        println!("no");
    }
}