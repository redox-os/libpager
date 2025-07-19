use ansi_control_codes::c0::{CR, LF};
use ansi_control_codes::parser::{Token, TokenStream};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, style};

use std::cmp;
use std::io::{self, Read, Write};

/// Start the pager over some string.
// TODO: as we need to parse control sequences anyways, allow interpreting / escaping stuff.
pub fn start<R, W>(stdin: R, stdout: W, title: &str, content: &str) -> io::Result<()>
where
    R: Read,
    W: IntoRawMode,
{
    let line_count = content.lines().count();
    let term_size = termion::terminal_size()?;

    let redraw = |stdout: &mut RawTerminal<W>, y: i32| -> io::Result<()> {
        write!(
            stdout,
            "{}{}{} {} {}..{}/{} {}{}{}",
            cursor::Goto(1, 1),
            style::Invert,
            clear::CurrentLine,
            title,
            y + 1,
            y + term_size.1 as i32 - 1,
            line_count,
            style::Reset,
            cursor::Goto(1, 2),
            clear::AfterCursor
        )?;
        for line in content
            .lines()
            .skip(y as usize)
            .take(term_size.1 as usize - 1)
        {
            let mut avail = term_size.0 as usize;
            for i in TokenStream::from(line) {
                match i {
                    Token::String(s) => {
                        if avail == 0 {
                            break;
                        }
                        let len = if let Some(new_avail) = avail.checked_sub(s.len()) {
                            avail = new_avail;
                            s.len()
                        } else {
                            core::mem::take(&mut avail)
                        };
                        write!(stdout, "{}", &s[..len])?;
                    }
                    Token::ControlFunction(ctrl) if ctrl == CR => {
                        // ignore these
                    }
                    Token::ControlFunction(ctrl) if ctrl == LF => {
                        write!(stdout, "\r{}", cursor::Down(1))?;
                        avail = 0;
                        break;
                    }
                    Token::ControlFunction(ctrl) => {
                        write!(stdout, "{}", ctrl)?;
                    }
                }
            }
            if avail != 0 {
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
            Key::PageDown | Key::Char(' ') => dy = term_size.1 as i32 - 1,
            Key::Char('d') => dy = (term_size.1 as i32 - 1) / 2,
            Key::Up | Key::Char('k') => dy = -1,
            Key::PageUp | Key::Char('b') => dy = -(term_size.1 as i32 - 1),
            Key::Char('u') => dy = -(term_size.1 as i32 - 1) / 2,
            Key::Char('q') => break,
            _ => (),
        }

        if dy != 0 {
            y = cmp::max(
                0,
                cmp::min(y + dy, line_count as i32 - (term_size.1 as i32 - 1)),
            );
            dy = 0;

            redraw(&mut stdout, y)?;
        }
    }

    write!(
        stdout,
        "{}{}{}{}",
        cursor::Goto(1, 1),
        style::Reset,
        clear::AfterCursor,
        cursor::Show
    )?;

    Ok(())
}
