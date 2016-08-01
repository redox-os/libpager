#![feature(question_mark)]

extern crate termion;

use termion::{style, cursor, clear};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::cmp;
use std::io::{self, Read, Write};

/// Start the pager over some string.
pub fn start<R, W>(stdin: R, stdout: W, title: &str, content: &str) -> io::Result<()>
    where R: Read,
          W: Write {
    let line_count = content.lines().count();
    let term_size = termion::terminal_size()?;

    let redraw = |stdout: &mut RawTerminal<W>, y: i32| -> io::Result<()> {
        write!(stdout, "{}{}{} {} {}..{}/{} {}{}{}",
                cursor::Goto(1, 1), style::Invert, clear::CurrentLine,
                title, y + 1, y + term_size.1 as i32, line_count,
                style::Reset, cursor::Goto(1, 2), clear::AfterCursor)?;
        for line in content.lines().skip(y as usize).take(term_size.1 as usize - 1) {
            if let Some(end) = line.char_indices().take(term_size.0 as usize).last() {
                write!(stdout, "{}\r{}", &line[..end.0], cursor::Down(1))?;
            } else {
                write!(stdout, "\r{}", cursor::Down(1))?;
            }
        }
        stdout.flush()?;
        Ok(())
    };

    let mut stdout = stdout.into_raw_mode()?;

    write!(stdout, "{}", cursor::Hide)?;

    let mut y = 0i32;
    let mut dy = 0;

    redraw(&mut stdout, y)?;

    for i in stdin.keys() {
        match i? {
            Key::Down | Key::Char('j') => dy = 1,
            Key::Up | Key::Char('k') => dy = -1,
            Key::Char('q') => break,
            _ => (),
        }

        if (dy > 0 || y != 0) && dy != 0 {
            y = cmp::max(0, cmp::min(y + dy, line_count as i32 - term_size.1 as i32));
            dy = 0;

            redraw(&mut stdout, y)?;
        }
    }

    write!(stdout, "{}", cursor::Show)?;

    Ok(())
}
