use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::Paragraph};
use std::fs::OpenOptions;
use std::io::{self, Write, stdout};

enum VimMode {
    Insert,
    Visual,
    Command,
    Normal,
}

fn main() -> io::Result<()> {
    // Setup: enter raw mode and alternate screen
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut command_buffer = String::with_capacity(3);
    let mut big_buffer: Vec<String> = Vec::new();
    let mut line = String::with_capacity(4096);

    // Main loop
    loop {
        // Draw
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

            let command_bar = Paragraph::new(command_buffer.clone());
            let text_lines: Vec<Line> = big_buffer.iter().map(|s| Line::from(s.clone())).collect();
            let file_buffer = Paragraph::new(text_lines);
            frame.render_widget(file_buffer, chunks[0]);
            frame.render_widget(command_bar, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char(c) => {
                    command_buffer.push(c);
                }
                KeyCode::Backspace => {
                    command_buffer.pop();
                }

                KeyCode::Enter => {
                    if command_buffer == ":q" {
                        break;
                    } else {
                        big_buffer.push(line.clone());
                        line = "".to_string();
                    }
                }
                _ => {}
            }
        }
    }

    // Teardown: restore terminal state
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn test() {
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
