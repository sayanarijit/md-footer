use std::io::Read;

pub fn format(index: usize, link: &str) -> String {
    format!("[{}]:{}", index + 1, link)
}

pub fn convert<R, F>(mut reader: R, formatter: F) -> String
where
    R: Read,
    F: Fn(usize, &str) -> String,
{
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
                let pointer = if let Some(position) =
                    collected_links.iter().position(|l| l == &link)
                {
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
                link.push(c as char);
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
            for b in formatter(i, link).bytes() {
                bytes.push(b);
            }
            bytes.push(b'\n');
        }
    }

    String::from_utf8(bytes).expect("failed to render")
}
