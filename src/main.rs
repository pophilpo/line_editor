use crossterm::{
    cursor::{position, MoveLeft, MoveRight, MoveToColumn, MoveToNextLine},
    event::read,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
    terminal::ScrollUp,
    ExecutableCommand, QueueableCommand, Result,
};

use std::io::{stdout, Stdout, Write};

fn print_mesasge(stdout: &mut Stdout, msg: &str) -> Result<()> {
    stdout
        .queue(Print("\n"))?
        .queue(MoveToColumn(1))?
        .queue(Print(msg))?
        .queue(Print("\n"))?
        .queue(MoveToColumn(1))?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut caret_pos: u16;
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;

    'repl: loop {
        stdout
            .execute(SetForegroundColor(Color::Green))?
            .execute(Print(">> :: "))?
            .execute(ResetColor)?;

        let (mut input_start_column, _) = position()?;
        input_start_column += 1;
        caret_pos = input_start_column;

        'input: loop {
            match read()? {
                Event::Key(KeyEvent { code, modifiers }) => match code {
                    KeyCode::Char(c) => {
                        if modifiers == KeyModifiers::CONTROL {
                            if c == 'c' {
                                stdout.queue(MoveToNextLine(1))?.queue(Print("exit"))?;
                                break 'repl;
                            }
                        }

                        let insertion_point = (caret_pos - input_start_column) as usize;
                        if insertion_point == buffer.len() {
                            stdout.queue(Print(c))?;
                        } else {
                            stdout
                                .queue(Print(c))?
                                .queue(Print(&buffer[insertion_point..]))?
                                .queue(MoveToColumn(caret_pos + 1))?;
                        }
                        stdout.flush()?;
                        buffer.insert(insertion_point, c);
                        caret_pos += 1;
                    }
                    KeyCode::Enter => {
                        if buffer == "exit()" {
                            break 'repl;
                        } else {
                            stdout.queue(ScrollUp(1))?.queue(MoveToColumn(1))?;
                            stdout.flush()?;
                            buffer.clear();
                            break 'input;
                        }
                    }
                    KeyCode::Left => {
                        if caret_pos > input_start_column {
                            stdout.queue(MoveLeft(1))?;
                            stdout.flush()?;
                            caret_pos -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if (caret_pos as usize) < (input_start_column as usize + buffer.len()) {
                            stdout.queue(MoveRight(1))?;
                            stdout.flush()?;
                            caret_pos += 1;
                        }
                    }

                    KeyCode::Backspace => {
                        let insertion_point = (caret_pos - input_start_column) as usize;

                        if insertion_point == buffer.len() && !buffer.is_empty() {
                            buffer.pop();
                            stdout
                                .queue(MoveLeft(1))?
                                .queue(Print(" "))?
                                .queue(MoveLeft(1))?;
                            stdout.flush()?;
                            caret_pos -= 1;
                        } else if insertion_point < buffer.len() {
                            buffer.remove(insertion_point - 1);
                            stdout
                                .queue(MoveLeft(1))?
                                .queue(Print(&buffer[(insertion_point - 1)..]))?
                                .queue(Print(" "))?
                                .queue(MoveToColumn(caret_pos))?;
                            stdout.flush()?;
                            caret_pos -= 1;
                        }
                    }
                    KeyCode::Delete => {
                        let insertion_point = (caret_pos - input_start_column) as usize;

                        if insertion_point < buffer.len() && !buffer.is_empty() {
                            buffer.remove(insertion_point);
                            stdout
                                .queue(Print(&buffer[insertion_point..]))?
                                .queue(Print(" "))?
                                .queue(MoveToColumn(caret_pos))?;

                            stdout.flush()?;
                        }
                    }

                    _ => {}
                },
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
