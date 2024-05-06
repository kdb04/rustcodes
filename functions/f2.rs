fn main(){
    call_by_value();
    call_by_ref();
}

fn call_by_ref(){
    let mut num:i32 = 5;
    println!("Value:{}", num);
    println!("Call by reference");
    Ref(& mut num);
    println!("Value:{}", num);
}

fn Ref(param_no: &mut i32){
    *param_no = 0;
}

fn call_by_value(){
    let mut num:i32 = 10;
    println!("Value:{}", num);
    println!("Call by value");
    val(num);
    println!("Value:{}", num);
}

fn val(mut param_no: i32){
    param_no = param_no*0;
}
