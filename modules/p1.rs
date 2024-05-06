pub mod movies{
    pub fn play(name:String){
        println!("Playing movie:{}", name);
    }
}

//Can use 'use movies::play'
fn main(){
    movies::play("Hangover".to_string());
}
