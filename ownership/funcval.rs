fn main(){
    let v = vec![1,2,3];
    let v2 = v;
    display(v2);
    //println!("in main {:?}", v2); error as v2 passed to display function
}

fn display(v:Vec<i32>){
    println!("Inside display {:?}", v);
}
