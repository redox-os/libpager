#![feature(question_mark)]

extern crate termion;

use termion::{style, cursor, clear};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{self, Read, Write};

/// Start the pager over some string.
pub fn start<R, W>(stdin: R, stdout: W, title: &str, content: &str) -> io::Result<()>
    where R: Read,
          W: Write {
    let mut stdout = stdout.into_raw_mode().unwrap();

    write!(stdout, "{}{}{}{} Welcome to pager: j/k/q {}{}", clear::All, cursor::Goto(1, 1),
           style::Invert, clear::CurrentLine, style::Reset, cursor::Hide)?;

    stdout.flush()?;

    let mut y = 0i16;
    let mut dy = 0;

    for i in stdin.keys() {
        match i.unwrap() {
            Key::Down | Key::Char('j') => dy = 1,
            Key::Up | Key::Char('k') => dy = -1,
            Key::Char('q') => break,
            _ => (),
        }

        if (dy > 0 || y != 0) && dy != 0 {
            y += dy;
            write!(stdout, "{}{}{} {} {}{}{}", cursor::Goto(1, 1), style::Invert, clear::CurrentLine,
                   title, style::Reset, cursor::Goto(1, 2), clear::AfterCursor)?;
            for line in content.lines().skip(y as usize).take(termion::terminal_size().unwrap().1 as usize) {
                write!(stdout, "{}\r{}", line, cursor::Down(1))?;
            }
            stdout.flush()?;
            dy = 0;
        }
    }

    write!(stdout, "{}", cursor::Show)?;

    Ok(())
}
