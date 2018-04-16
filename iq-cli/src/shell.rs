//! Terminal handling code
//!
//! Portions of this code are borrowed from Cargo

use std::fmt;
use std::io;
use std::io::prelude::*;
use term::{self, Attr, TerminfoTerminal};
use term::Terminal as RawTerminal;
use term::color::{Color, BLACK};

/// Color configuration
#[derive(Clone, Copy, PartialEq)]
pub enum ColorConfig {
    /// Pick colors automatically based on whether we're using a TTY
    Auto,

    /// Always use colors
    Always,

    /// Never use colors
    Never,
}

impl fmt::Display for ColorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorConfig::Auto => "auto",
            ColorConfig::Always => "always",
            ColorConfig::Never => "never",
        }.fmt(f)
    }
}

/// Shell configuration options
#[derive(Clone, Copy)]
pub struct ShellConfig {
    /// Color configuration
    pub color_config: ColorConfig,

    /// Are we using a TTY?
    pub tty: bool,
}

enum Terminal {
    NoColor(Box<Write + Send>),
    Colored(Box<RawTerminal<Output = Box<Write + Send>> + Send>),
}

/// Terminal shell we interact with
pub struct Shell {
    terminal: Terminal,
    config: ShellConfig,
}

impl Shell {
    /// Create a new shell
    pub fn create<T: FnMut() -> Box<Write + Send>>(mut out_fn: T, config: ShellConfig) -> Shell {
        let terminal = Shell::get_term(out_fn()).unwrap_or_else(|_| Terminal::NoColor(out_fn()));
        Shell { terminal, config }
    }

    /// Get the shell's Terminal
    fn get_term(out: Box<Write + Send>) -> term::Result<Terminal> {
        Ok(Shell::get_terminfo_term(out))
    }

    /// Get the terminfo Terminal
    fn get_terminfo_term(out: Box<Write + Send>) -> Terminal {
        match ::term::terminfo::TermInfo::from_env() {
            Ok(ti) => {
                let term = TerminfoTerminal::new_with_terminfo(out, ti);
                if term.supports_color() {
                    Terminal::Colored(Box::new(term))
                } else {
                    Terminal::NoColor(term.into_inner())
                }
            }
            Err(_) => Terminal::NoColor(out),
        }
    }

    /// Say something with the given color
    pub fn say<T: ToString>(&mut self, message: &T, color: Color) -> term::Result<()> {
        self.reset()?;

        if color != BLACK {
            self.fg(color)?;
        }

        write!(self, "{}\n", message.to_string())?;
        self.reset()?;
        self.flush()?;

        Ok(())
    }

    /// Say a status message with the given color
    pub fn say_status<T, U>(
        &mut self,
        status: T,
        message: U,
        color: Color,
        justified: bool,
    ) -> term::Result<()>
    where
        T: fmt::Display,
        U: fmt::Display,
    {
        self.reset()?;

        if color != BLACK {
            self.fg(color)?;
        }

        if self.supports_attr(Attr::Bold) {
            self.attr(Attr::Bold)?;
        }

        if justified {
            write!(self, "{:>12}", status.to_string())?;
        } else {
            write!(self, "{}", status)?;
        }

        self.reset()?;
        write!(self, " {}\n", message)?;
        self.flush()?;

        Ok(())
    }

    fn fg(&mut self, color: Color) -> term::Result<bool> {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref mut c) if colored => c.fg(color)?,
            _ => return Ok(false),
        }

        Ok(true)
    }

    fn attr(&mut self, attr: Attr) -> term::Result<bool> {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref mut c) if colored => c.attr(attr)?,
            _ => return Ok(false),
        }

        Ok(true)
    }

    fn supports_attr(&self, attr: Attr) -> bool {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref c) if colored => c.supports_attr(attr),
            _ => false,
        }
    }

    fn reset(&mut self) -> term::Result<()> {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref mut c) if colored => c.reset()?,
            _ => (),
        }

        Ok(())
    }

    fn colored(&self) -> bool {
        self.config.tty && ColorConfig::Auto == self.config.color_config
            || ColorConfig::Always == self.config.color_config
    }
}

impl Write for Shell {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.terminal {
            Terminal::Colored(ref mut c) => c.write(buf),
            Terminal::NoColor(ref mut n) => n.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.terminal {
            Terminal::Colored(ref mut c) => c.flush(),
            Terminal::NoColor(ref mut n) => n.flush(),
        }
    }
}
