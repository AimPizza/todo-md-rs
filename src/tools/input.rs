use std::io::{self, Write};

pub fn readinput(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    print!("{prompt}");
    io::stdout().flush()?;
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    // remove trailing newline
    let input = buffer.trim().to_string();

    Ok(input)
}

/* // idk if this does anything useful
fn main() {
    let rawinput = readinput("write anything: ");
    match rawinput {
        Ok(something) => {
            let parts = something.split_whitespace();
            for something in parts {
                println!("element: \"{something}\"");
            }
            println!("you typed: {something}");
        }
        Err(err) => {
            eprintln!("Error: {err}")
        }
    }
}
*/
