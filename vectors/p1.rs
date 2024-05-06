fn main(){
    let mut v1 = Vec::new();
    v1.push(20);
    v1.push(30);
    v1.push(40);
    println!("Size:{}", v1.len());
    println!("{:?}", v1);
    let v2 = vec![1,2,3];
    println!("{:?}", v2);
    v1.remove(1);
    println!("{:?}", v1);
    if v1.contains(&20){
        println!("Found");
    }
    println!("{:?}", v1);
    println!("{:?}", v1[0]);
    println!("{:?}", v1[1]);
    for i in &v1{
        println!("{}", i);
    }
}
