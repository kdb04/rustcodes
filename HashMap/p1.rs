use std::collections::HashMap;
fn main(){
    let mut StateCodes = HashMap::new();
    StateCodes.insert("KA", "Karnataka");
    StateCodes.insert("KL", "Kerala");
    println!("{:?}", StateCodes);
    println!("{}", StateCodes.len());
    match StateCodes.get(&"KA") {
        Some(value) => {
            println!("{}", value);
        },
        None => {
            println!("Not found");
        }
    }
    for (key,val) in StateCodes.iter(){
        println!("{},{}", key,val);
    }
    if StateCodes.contains_key(&"KA"){
        println!("Found");
    }
    StateCodes.remove(&"KL");
    println!("{}", StateCodes.len());
}
