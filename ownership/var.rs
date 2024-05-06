fn main(){
    let v = vec![1,2,3];
    let v2 = v; //Transfer of ownership to v2
    println!("{:?}", v2);
    //println!("{:?}" v);  error 
}
