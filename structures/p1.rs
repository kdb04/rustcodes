struct Employee{
    name:String,
    company:String,
    age:u32
}

fn main(){
    let mut emp1 = Employee{
        company:String::from("A"),
        name:String::from("B"),
        age:50
    };
    emp1.age = 40;
    println!("{}, {}, {}", emp1.company, emp1.name, emp1.age);
}
