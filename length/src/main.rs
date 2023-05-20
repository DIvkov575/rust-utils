use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "file" {
        // let full_path = &args[0] + &args[2];
        let contents: String = fs::read_to_string(&args[2]).unwrap();
        let a = contents.len();
        print!("{:#?}", a);
    } else {
        print!("{:#?}", args[1].len());
    }
}
