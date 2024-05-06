#[derive(Debug)]
enum Gender{
    Name(String), Usr_ID(i32)
}

fn main(){
    let p1 = Gender::Name(String::from("A"));
    let p2 = Gender::Usr_ID(100);
    println!("{:?}", p1);
    println!("{:?}", p2);

    match p1{
        Gender::Name(val)=>{
            println!("{}", val);
        }
        Gender::Usr_ID(val)=>{
            println!("{}", val);
        }
    }
}
