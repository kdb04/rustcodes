#[derive(Debug)]
enum Gender{
    Male, Female
}

#[derive(Debug)]
struct Person{
    name:String,
    gender:Gender
}

fn main(){
    let p1 = Person{
        name:String::from("A"),
        gender:Gender::Male
    };
    let p2 = Person{
        name:String::from("B"),
        gender:Gender::Female
    };
    println!("{:?}", p1);
    println!("{:?}", p2);
}
