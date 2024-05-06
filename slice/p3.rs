fn main(){
    let mut data = [10,20,30,40,50];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn use_slice(slice: &mut [i32]){
    println!("Length:{}", slice.len());
    println!("{:?}", slice);
    slice[1] = 0;
}
