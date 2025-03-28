fn main() {
    println!("reviewing");
    let username: String="AlireZa".to_owned();
    //println!("{username}")
    let password:String=String::from("Alichi1234");
    //println!("{password}");
    println!("===login===");
    println!("");
    loop{     
    println!("please enter your username:");
    let mut input_username:String=String::new();
    std::io::stdin().read_line(&mut input_username);
    if username.to_lowercase()!=input_username.trim().to_lowercase() {  //.to_lowercase() makes the string lowercase, it dosn't care about capital letters
        println!("username is not correct");
        continue;
    }else{
        println!("username is correct");
        break;

    }
}
    loop{
    println!("please enter your password:");
    let mut input_password:String=String::new();
    std::io::stdin().read_line(&mut input_password);
    if password!=input_password.trim() {
        println!("password is not correct");
        continue;
    }else{
        println!("password is correct");
        break;

    }
}


}
