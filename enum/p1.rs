#[derive(Debug)]
enum Gender{
    Male, Female
}

fn main(){
    let male = Gender::Male;
    let female = Gender::Female;
    println!("{:?}", male);
    println!("{:?}", female);
}
