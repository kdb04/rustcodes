use std::io;

fn main(){
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut num1).expect("Invalid value");
    println!("Enter the second number:");
    io::stdin().read_line(&mut num2).expect("Invalid value");
    let num1:f64 = num1.trim().parse().expect("Invalid value");
    let num2:f64 = num2.trim().parse().expect("Invalid value");
    let add = num1 + num2;
    let sub = num1 - num2;
    let mul = num1 * num2;
    let div = num1 / num2;
    let modulus = num1 % num2;
    println!("Addition:{}", add);
    println!("Subtraction:{}", sub);
    println!("Multiplication:{}", mul);
    println!("Division:{}", div);
    println!("Modulus:{}", modulus);
}
