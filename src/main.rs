use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

fn convert(mut reader: File) -> String {
    let mut is_codeblock = false;
    let mut is_hiperlink = false;
    let mut collected_links: Vec<String> = vec![];
    let mut links_stack: Vec<String> = vec![];
    let mut last_byte = b'\0';
    let mut bytes = vec![];
    let mut byte = [0u8];

    while reader.read(&mut byte).expect("failed to read file") != 0 {
        let c = byte[0];
        if c == b'`' && is_codeblock {
            is_codeblock = false;
            bytes.push(c);
        } else if c == b'`' && !is_codeblock {
            is_codeblock = true;
            bytes.push(c);
        } else if c == b'[' && !is_codeblock {
            links_stack.push("".into());
            bytes.push(c);
        } else if c == b'(' && last_byte == b']' && !is_codeblock {
            is_hiperlink = true;
        } else if c == b')' && !is_codeblock && is_hiperlink {
            is_hiperlink = false;
            if let Some(link) = links_stack.pop() {
                let pointer =
                    if let Some(position) = collected_links.iter().position(|l| l == &link) {
                        position + 1
                    } else {
                        collected_links.push(link);
                        collected_links.len()
                    };

                bytes.push(b'[');
                for b in pointer.to_string().bytes() {
                    bytes.push(b.clone());
                }
                bytes.push(b']');
            } else {
                bytes.push(c);
            }
        } else if !is_codeblock && is_hiperlink {
            if let Some(link) = links_stack.last_mut() {
                if is_hiperlink {
                    link.push(c as char);
                } else {
                    bytes.push(c);
                };
            } else {
                bytes.push(c);
            }
        } else {
            bytes.push(c);
        };
        last_byte = c;
    }

    if !collected_links.is_empty() {
        bytes.push(b'\n');
        bytes.push(b'\n');
        for (i, link) in collected_links.iter().enumerate() {
            for b in format!("[{}]:{}\n", i + 1, link).bytes() {
                bytes.push(b);
            }
        }
    }

    String::from_utf8(bytes).expect("failed to render")
}

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
