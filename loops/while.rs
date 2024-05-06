fn main(){
    let mut x = 0;
    while x<10{
        x+=1;
        println!("Inside loop: {}", x);
    }
    println!("Outside loop: {}", x);
}
