use text_io::read;
fn main() {
    let stuff:String = read!("{}\n");
    let mut output:Vec<String> = Vec::new();
    for i in stuff.chars() {
        if i.is_whitespace() {
            output.push("37".parse().unwrap());
        }
        else {
            output.push(char::to_digit(i, 36).unwrap().to_string());
        }
    }
    println!("{}", output.join(""));
}
