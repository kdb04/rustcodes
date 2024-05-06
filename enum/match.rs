enum CarType{
    Hatch,
    Sedan,
    SUV
}

fn car_size(car:CarType){
    match car{
        CarType::Hatch=>{
            println!("Small");
        },
        CarType::Sedan=>{
            println!("Medium");
        },
        CarType::SUV=>{
            println!("Large");
        }
    }
}

fn main(){
    car_size(CarType::SUV);
    car_size(CarType::Sedan);
    car_size(CarType::Hatch);
}
