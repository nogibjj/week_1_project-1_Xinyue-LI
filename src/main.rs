use std::io;

fn main() {
    println!("Plese enter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("please enter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();
    let age: u32 = age.trim().parse().unwrap();

    println!("What is your favorite color?");
    let mut color = String::new();
    io::stdin().read_line(&mut color).unwrap();
    let color = color.trim();

    println!("Hello, {}! You are {} years old and your favorite color is {}", name, age, color);
}


// fn main() {
//     println!("Hello, world!");

// }

// use std::io;

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let num = input.trim().parse::<i32>().unwrap();
//     println!("The square of {} is {}", num, num * num);
// }

