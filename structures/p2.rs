struct Employee{
    name:String,
    company:String,
    age:u32
}

fn main(){
    let emp1 = Employee{
        name:String::from("A"),
        company:String::from("B"),
        age:50
    };
    let emp2 = Employee{
        name:String::from("C"),
        company:String::from("D"),
        age:60
    };
    display(emp1);
    display(emp2);
}

fn display(emp:Employee){
    println!("{} {} {}", emp.name, emp.company, emp.age);
}
