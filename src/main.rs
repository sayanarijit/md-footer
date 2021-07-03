use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Default)]
struct Link {
    text: String,
    href: String,
}

fn convert(reader: BufReader<File>) -> String {
    let mut is_codeblock = false;
    let mut is_hiperlink = false;
    let mut collected_links = vec![];
    let mut links_stack = vec![];
    let mut last_char = '\0';
    let mut lines: Vec<String> = vec![];

    for maybe_line in reader.lines() {
        match maybe_line {
            Ok(line) => {
                let mut chars = vec![];
                for c in line.chars() {
                    if c == '`' && is_codeblock {
                        is_codeblock = false;
                        chars.push(c);
                    } else if c == '`' && !is_codeblock {
                        is_codeblock = true;
                        chars.push(c);
                    } else if c == '[' && !is_codeblock {
                        links_stack.push(Link::default());
                        chars.push(c);
                    } else if c == '(' && last_char == ']' && !is_codeblock {
                        is_hiperlink = true;

                        chars.push('[');
                        for c in (collected_links.len() + 1).to_string().chars() {
                            chars.push(c);
                        }
                        chars.push(']');
                    } else if c == ')' && !is_codeblock && is_hiperlink {
                        is_hiperlink = false;
                        if let Some(link) = links_stack.pop() {
                            collected_links.push(link);
                        }
                    } else if let Some(link) = links_stack.last_mut() {
                        if is_hiperlink {
                            link.href.push(c);
                        } else {
                            link.text.push(c);
                            chars.push(c);
                        };
                    } else {
                        chars.push(c);
                    };
                    last_char = c;
                }
                lines.push(chars.into_iter().collect());
            }
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
        }
    }

    lines.push("".into());
    lines.push("".into());
    for (i, link) in collected_links.iter().enumerate() {
        lines.push(format!("[{}]:{}", i + 1, link.href));
    }
    lines.join("\n")
}

fn main() {
    let path = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "/dev/stdin".into());

    match fs::File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let result = convert(reader);
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
}
