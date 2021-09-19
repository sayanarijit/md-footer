use std::env;
use std::fs;
use md_footer::convert;

fn main() {
    let path = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "/dev/stdin".into());

    match fs::File::open(path) {
        Ok(file) => {
            let result = convert(file);
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
}
