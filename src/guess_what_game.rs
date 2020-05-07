use std::io;

fn guess_what_game() {
    println!("Hello, world! Start guess what game!");
    //此处 println! 是一个Rust 宏（macro），不是一个普通函数

    println!("Please input your guess.");

    //mut 意思为是一个变量，此处注意如果没有 mut 则表示这是一个常量
    let mut guess = String::new();

    //& 的意思是引用，不就是指针？？？
    //read_line返回的是一个Result的枚举类型，成员为Ok Err
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed is {}", guess);

}