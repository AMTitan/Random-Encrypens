use text_io::read;
fn main() {
    println!("What do you want to encrypt in?
1: numbers
2: random readable");
    let types:i32 = read!("{}\n");
    println!("what do you want encrypted?");
    let input:String = read!("{}\n");
    match types {
        1 => println!("{}", numbers(input)),
        2 => println!("{}", random_readable(input)),
        _ => {}
    }
}

fn numbers(input:String) -> String {
    let mut output:Vec<String> = Vec::new();
    for i in input.chars() {
        if i.is_whitespace() {
            output.push("37".parse().unwrap());
        }
        else {
            output.push(char::to_digit(i, 36).unwrap().to_string());
        }
    }
    output.join("")
}

fn random_readable(input:String) -> String {
    use rand::Rng;
    let mut output:Vec<String> = Vec::new();
    for i in input.split_ascii_whitespace() {
        let mut to_push:String = i.chars().nth(0).unwrap().to_string();
        if i.chars().count() > 2 {
            let mut letters:Vec<char> = i.chars().collect();
            letters.remove(0);
            letters.remove(letters.len()-1);
            for _x in 0..i.chars().count()-2 {
                let n = rand::thread_rng().gen_range(0..letters.len());
                to_push.push_str(letters[n].to_string().as_str());
                letters.remove(n);
            }
            to_push.push_str(i.chars().last().unwrap().to_string().as_str());
        }
        else if i.chars().count() > 1 {
            to_push.push_str(i.chars().nth(1).unwrap().to_string().as_str());
        }
        output.push(to_push);
    }
    output.join(" ")
}
