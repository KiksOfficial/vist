use std::fs::OpenOptions;
use std::io;
use std::io::Write;

fn main() {
    println!("Vist: a Vim-like text editor written in rust");
    println!("Currently just takes user input and stuff");

    let mut big_buffer: Vec<String> = Vec::new();

    loop {
        print!("Enter sth: ");
        io::stdout().flush().unwrap();

        let mut line = String::with_capacity(4096);
        let stdin_ = io::stdin();

        match stdin_.read_line(&mut line) {
            Ok(_) => {
                if handle_exit(line.trim()) {
                    break;
                } else if line.trim() == ":w" {
                    let _ = save_to_disk(&big_buffer);
                } else {
                    big_buffer.push(line.trim_end().to_string());
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn handle_exit(sisend: &str) -> bool {
    sisend == ":q"
}

fn save_to_disk(document: &[String]) -> io::Result<usize> {
    let document = document.join("\n");
    let mut buffer = OpenOptions::new().create(true).append(true).open("test")?;
    let bytes = document.as_bytes();

    let mut pos = 0;

    while pos < bytes.len() {
        let bytes_written = buffer.write(&bytes[pos..])?;
        pos += bytes_written;
    }

    Ok(pos)
}
