fn main(){
    let data = [10,20,30,40,50];
    use_slice(&data[1..4]);
}

fn use_slice(slice:&[i32]){
    println!("Length:{}", slice.len());
    println!("{:?}", slice);
}
