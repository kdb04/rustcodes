fn main(){
    let a:f32 = 1.0/3.0;
    let b:f64 = 1.0/3.0;
    let c = 10.00;
    let d = 10.00/5.00;
    let e = 10.00;
    println!("a:{}", a);//0.33333334
    println!("b:{}", b);//0.3333333333333333
    println!("c:{}", c);//10 not 10.00 when type not specified
    println!("d:{}", d);//2 not 2.00
    println!("e:{:.2}", e);//10.00
    println!("a:{:.2}", a);//0.33
}
