use minigrep::Grep;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let grep = Grep::from_args(&args).unwrap();
    for index in grep {
        println!("{}",index);
    }
}


