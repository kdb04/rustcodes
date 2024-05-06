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
        age:30
    };
    let elder = elder(emp1, emp2);
    display(elder);
}

fn elder(emp1:Employee, emp2:Employee)->Employee{
    if(emp1.age>emp2.age){
        return emp1;
    }
    else{
        return emp2;
    }
}

fn display(emp:Employee){
    println!("{} {} {}", emp.name, emp.company, emp.age);
}
