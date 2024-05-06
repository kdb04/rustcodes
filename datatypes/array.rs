fn main(){
    let arr1:[i32;4] = [1,2,3,4];
    println!("Array: {:?}", arr1);
    let arr2 = [1,2,3,4];
    println!("Array:{:?}", arr2);
    let arr3:[i32;4] = [7;4];
    println!("Array:{:?}", arr3);
    for i in 0..4{
        println!("Index:{}, Value:{}", i, arr1[i]);
    }
    for j in arr2.iter(){
        println!("Value:{}", j);
    }
    let mut arr4:[i32;4] = [10,20,30,40];
    arr4[1] = 0;
    println!("{:?}", arr4);
    let arr5 = [10,20,30];
    println!("{:?}", arr5);
    passbyvalue(arr5);
    println!("Inside main: {:?}", arr5);
    let mut arr6 = [10,20,30];
    passbyref(&mut arr6);
    println!("Inside main:{:?}", arr6);
}

fn passbyvalue(mut arr:[i32;3]){
    for i in 0..3{
        arr[i] = 0;
    }
    println!("Inside function:{:?}", arr);
}

fn passbyref(arr: &mut[i32; 3]){
    for i in 0..3{
        arr[i] = 0;
    }
    println!("Inside function: {:?}", arr);
}


