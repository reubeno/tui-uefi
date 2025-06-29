#![no_std]

use core::fmt::{self, Write};

use ratatui::prelude::backend::ClearType;
use thiserror::Error;
use uefi::{boot::ScopedProtocol, proto::console};

/// Error returned by the [`UefiOutputBackend`]
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("failed to set cursor: {inner}")]
    SetCursorPosition { inner: uefi::Error },
    #[error("failed to set color: {inner}")]
    SetColor { inner: uefi::Error },
    #[error("failed to write character: {inner}")]
    WriteCharacter { inner: fmt::Error },
    #[error("failed to clear: {inner}")]
    Clear { inner: uefi::Error },
    #[error("failed to get current mode: {inner}")]
    GetCurrentMode { inner: uefi::Error },
    #[error("No current mode available")]
    NoCurrentMode,
    #[error("clear_type [{clear_type:?}] not supported with this backend")]
    UnsupportedClear { clear_type: ClearType },
}

impl Error {
    fn set_cursor_position(inner: uefi::Error) -> Self {
        Error::SetCursorPosition { inner }
    }

    fn set_color(inner: uefi::Error) -> Self {
        Error::SetColor { inner }
    }

    fn write_character(inner: fmt::Error) -> Self {
        Error::WriteCharacter { inner }
    }

    fn clear(inner: uefi::Error) -> Self {
        Error::Clear { inner }
    }

    fn get_current_mode(inner: uefi::Error) -> Self {
        Error::GetCurrentMode { inner }
    }

    fn no_current_mode() -> Self {
        Error::NoCurrentMode
    }

    fn unsupported_clear(clear_type: ClearType) -> Self {
        Self::UnsupportedClear { clear_type }
    }
}

/// Implements a backend for the `ratatui` crate suitable for use in a UEFI application
/// or loader.
pub struct UefiOutputBackend {
    output: ScopedProtocol<console::text::Output>,
}

impl UefiOutputBackend {
    pub fn new(output: ScopedProtocol<console::text::Output>) -> Self {
        Self { output }
    }
}

fn to_uefi_color(color: ratatui::style::Color) -> Option<console::text::Color> {
    match color {
        ratatui::style::Color::Black => Some(console::text::Color::Black),
        ratatui::style::Color::Red => Some(console::text::Color::Red),
        ratatui::style::Color::Green => Some(console::text::Color::Green),
        ratatui::style::Color::Yellow => Some(console::text::Color::Yellow),
        ratatui::style::Color::Blue => Some(console::text::Color::Blue),
        ratatui::style::Color::Magenta => Some(console::text::Color::Magenta),
        ratatui::style::Color::Cyan => Some(console::text::Color::Cyan),
        ratatui::style::Color::Gray => Some(console::text::Color::LightGray),
        ratatui::style::Color::DarkGray => Some(console::text::Color::DarkGray),
        ratatui::style::Color::LightRed => Some(console::text::Color::LightRed),
        ratatui::style::Color::LightGreen => Some(console::text::Color::LightGreen),
        ratatui::style::Color::LightYellow => Some(console::text::Color::Yellow),
        ratatui::style::Color::LightBlue => Some(console::text::Color::LightBlue),
        ratatui::style::Color::LightMagenta => Some(console::text::Color::LightMagenta),
        ratatui::style::Color::LightCyan => Some(console::text::Color::LightCyan),
        ratatui::style::Color::White => Some(console::text::Color::White),
        ratatui::style::Color::Rgb(..)
        | ratatui::style::Color::Indexed(_)
        | ratatui::style::Color::Reset => None,
    }
}

impl ratatui::backend::Backend for UefiOutputBackend {
    type Error = Error;

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Error>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>,
    {
        for (x, y, cell) in content {
            let mut fg = to_uefi_color(cell.fg).unwrap_or(console::text::Color::White);
            let mut bg = to_uefi_color(cell.bg).unwrap_or(console::text::Color::Black);

            if cell.modifier.contains(ratatui::style::Modifier::REVERSED) {
                // Swap foreground and background colors.
                core::mem::swap(&mut fg, &mut bg);
            }

            self.output
                .set_cursor_position(x as usize, y as usize)
                .map_err(Error::set_cursor_position)?;

            self.output.set_color(fg, bg).map_err(Error::set_color)?;

            self.output
                .write_str(cell.symbol())
                .map_err(Error::write_character)?;
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Error> {
        // Not supported on all platforms.
        let _ = self.output.enable_cursor(false);

        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Error> {
        // Not supported on all platforms.
        let _ = self.output.enable_cursor(true);

        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<ratatui::prelude::Position, Error> {
        let (col, row) = self.output.cursor_position();

        Ok(ratatui::prelude::Position {
            x: col as u16,
            y: row as u16,
        })
    }

    fn set_cursor_position<P: Into<ratatui::prelude::Position>>(
        &mut self,
        position: P,
    ) -> Result<(), Error> {
        let pos = position.into();

        self.output
            .set_cursor_position(pos.x as usize, pos.y as usize)
            .map_err(Error::set_cursor_position)
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.output.clear().map_err(Error::clear)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Error> {
        match clear_type {
            ClearType::All => self.clear(),
            ClearType::AfterCursor
            | ClearType::BeforeCursor
            | ClearType::CurrentLine
            | ClearType::UntilNewLine => Err(Error::unsupported_clear(clear_type)),
        }
    }

    fn size(&self) -> Result<ratatui::prelude::Size, Error> {
        let mode = self
            .output
            .current_mode()
            .map_err(Error::get_current_mode)?;

        let mode = mode.ok_or_else(Error::no_current_mode)?;

        Ok(ratatui::prelude::Size {
            width: mode.columns() as u16,
            height: (mode.rows() - 2) as u16,
        })
    }

    fn window_size(&mut self) -> Result<ratatui::backend::WindowSize, Error> {
        let size = self.size()?;

        // TODO: Fill out pixel dimensions?
        Ok(ratatui::backend::WindowSize {
            columns_rows: size,
            pixels: ratatui::prelude::Size {
                width: 0,
                height: 0,
            },
        })
    }

    fn flush(&mut self) -> Result<(), Error> {
        // No-op?
        Ok(())
    }
}
