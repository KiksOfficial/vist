use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use ratatui::widgets::{self, Paragraph};
use std::{env, fs::OpenOptions};
use std::{
    fs,
    io::{self, Write, stdout},
};

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
    let mut big_buffer = String::with_capacity(4096);
    let mut command_mode = false;

    let mut line_lens: Vec<usize> = vec![0];
    let mut current_line: usize = 0;

    let argumenid: Vec<String> = env::args().collect();
    if argumenid.len() < 2 {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        return Ok(());
    }

    let mut cursor_x: u16 = 0;
    let mut cursor_y: u16 = 0;

    let filename = &argumenid[1];
    if fs::exists(filename)? {
        big_buffer = fs::read_to_string(filename)?;
        line_lens.clear();

        let mut len_in_line: usize = 0;
        for ch in big_buffer.chars() {
            if ch == '\n' {
                line_lens.push(len_in_line);
                len_in_line = 0;
                current_line += 1;
            } else {
                len_in_line += 1;
            }
        }
        line_lens.push(len_in_line);
        cursor_x = len_in_line as u16;
        cursor_y = current_line as u16;
    }

    // Main loop
    loop {
        // Draw
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

            let block_file = widgets::Block::default().borders(widgets::Borders::ALL);
            let inner_file_area = block_file.inner(chunks[0]);

            let command_bar = Paragraph::new(&*command_buffer);
            let file_display = Paragraph::new(&*big_buffer);

            frame.render_widget(block_file, chunks[0]);
            frame.render_widget(file_display, inner_file_area);
            frame.render_widget(command_bar, chunks[1]);

            if command_mode {
                frame.set_cursor_position(Position {
                    x: chunks[1].x + command_buffer.len() as u16,
                    y: chunks[1].y,
                });
            } else {
                frame.set_cursor_position(Position {
                    x: inner_file_area.x + cursor_x,
                    y: inner_file_area.y + cursor_y,
                });
            }
        })?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            if command_mode {
                match key.code {
                    KeyCode::Char(c) => {
                        command_buffer.push(c);
                        cursor_x += 1;
                    }
                    KeyCode::Backspace => {
                        command_buffer.pop();
                    }
                    KeyCode::Enter => {
                        if let Ok(return_) =
                            handle_command_buffer(&command_buffer, &big_buffer, filename)
                            && return_ == 1
                        {
                            break;
                        }
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char(':') => {
                        command_mode = true;
                        command_buffer.push(':');
                    }
                    KeyCode::Char(c) => {
                        big_buffer.push(c);
                        cursor_x += 1;
                        line_lens[current_line] += 1;
                    }
                    KeyCode::Backspace => {
                        if big_buffer.pop().is_some() {
                            if cursor_x > 0 {
                                cursor_x -= 1;
                                line_lens[current_line] -= 1;
                            } else if cursor_y > 0 {
                                cursor_y -= 1;
                                current_line -= 1;
                                cursor_x = line_lens[current_line] as u16;
                            }
                        }
                    }
                    KeyCode::Esc => {
                        command_mode = false;
                        command_buffer.clear();
                    }

                    KeyCode::Enter => {
                        big_buffer.push('\n');
                        cursor_y += 1;
                        cursor_x = 0;
                        current_line += 1;
                        line_lens.push(0);
                    }
                    _ => {}
                }
            }
        }
    }

    // Teardown: restore terminal state
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_command_buffer(sisend: &str, faili_sisu: &str, filename: &str) -> io::Result<u8> {
    for char_ in sisend.chars() {
        match char_ {
            'w' => {
                save_to_disk(faili_sisu, filename)?;
            }
            'q' => return Ok(1),
            _ => {}
        }
    }
    Ok(0)
}

fn save_to_disk(faili_sisu: &str, filename: &str) -> io::Result<usize> {
    let mut buffer = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;
    let bytes = faili_sisu.as_bytes();

    let mut pos = 0;

    while pos < bytes.len() {
        let bytes_written = buffer.write(&bytes[pos..])?;
        pos += bytes_written;
    }

    Ok(pos)
}
