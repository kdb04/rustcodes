fn main(){
    let tuple:(i32, bool, f64) = (30, true, 6.9);
    println!("{:?}", tuple.0);
    println!("{:?}", tuple.1);
    println!("{:?}", tuple.2);
    print(tuple);
}

fn print(x:(i32, bool, f64)){
    println!("Inside print");
    let (age, ismale, cgpa) = x; //Destructing - unpacking values of tuple
    println!("age:{}, ismale?:{}, cgpa:{}", age, ismale, cgpa);
}
