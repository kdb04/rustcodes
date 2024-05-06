struct Data<T>{
    value:T,
}

fn main(){
    let t1:Data<i32> = Data{value:350};
    println!("value:{}", t1.value);
    let t2:Data<String> = Data{value:"A".to_string()};
    println!("value:{}", t2.value);
}
