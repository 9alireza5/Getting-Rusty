fn main() {
    let grades = [12, 15, 20, 4, 18];
    let mut sum = 0;
    for grade in grades.iter() {
        sum += grade;
    }
    let average = sum as f64 / grades.len() as f64;
    println!("The average of grades as a floating number is: {:.1}", average);
    //shoing as intiger
        let grades = [12, 15, 20, 4, 18];
        let mut sum = 0;
        for grade in grades.iter() {
            sum += grade;
        }
        let average = sum / grades.len();
        println!("The average of grades as an intiger number is: {}", average);
    }