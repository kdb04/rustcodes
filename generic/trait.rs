fn main(){
    let b1 = Book{
        id:1000,
        Name:"A"
    };
    b1.print();
}

struct Book{
    id:u32,
    Name:&'static str 
}

trait Printable{
    fn print(&self);
}

impl Printable for Book{
    fn print(&self){
        println!("{} {}", self.id, self.Name);
    }
}
