fn main(){
    let state_code = "KA";
    let state = match state_code{
        "KA" => "Karnataka",
        "KL" => "Kerala",
        "TN" => "Tamil Nadu",
        _ => "Unknown"
    };
    println!("State name is {}", state);
}
