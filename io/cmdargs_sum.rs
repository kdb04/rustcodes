fn main(){
    let cmd_line = std::env::args();
    println!("No of elements {}", cmd_line.len());
    let mut sum = 0;
    let mut has_read_first_arg = false;
    for arg in cmd_line{
        if has_read_first_arg{
            sum += arg.parse::<i32>().unwrap();
        }
        has_read_first_arg = true;
    }
    println!("{}", sum);
}
