use std::collections::HashSet;
fn main(){
    let mut names = HashSet::new();
    names.insert("A");
    names.insert("B");
    names.insert("C");
    names.insert("A"); //Duplicates ignored
    println!("{:?}", names);
    println!("{}", names.len());
    for i in names.iter(){
        println!("{}", i);
    }
    match names.get(&"A"){
        Some(value)=>{
            println!("Found");
        }
        None=>{
            println!("Not found");
        }
    }
    if names.contains(&"B"){
        println!("Found");
    }
    names.remove(&"C");
    println!("{}", names.len());

}
