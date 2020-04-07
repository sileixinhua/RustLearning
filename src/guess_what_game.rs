use std::io;

fn guess_what_game() {

    println!("Hello, world! Start guess what game!");
    //此处 println! 是一个Rust 宏（macro），不是一个普通函数
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed is {}", guess);


}