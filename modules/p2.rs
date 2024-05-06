pub mod Movies{
    pub mod English{
        pub mod comedy{
            pub fn play(name:String){
                println!("Playing comedy movie: {}", name);
            }
        }
    }
}

//Shorthand use Movies::English::comedy::play;
fn main(){
    Movies::English::comedy::play("Hangover".to_string());
}
