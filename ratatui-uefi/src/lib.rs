use std::fmt::Write;

use uefi::{boot::ScopedProtocol, proto::console};

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
    fn draw<'a, I>(&mut self, content: I) -> std::io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>,
    {
        for (x, y, cell) in content {
            let mut fg = to_uefi_color(cell.fg).unwrap_or(console::text::Color::White);
            let mut bg = to_uefi_color(cell.bg).unwrap_or(console::text::Color::Black);

            if cell.modifier.contains(ratatui::style::Modifier::REVERSED) {
                // Swap foreground and background colors.
                std::mem::swap(&mut fg, &mut bg);
            }

            self.output
                .set_cursor_position(x as usize, y as usize)
                .map_err(|_| std::io::Error::other("Failed to set cursor"))?;

            self.output
                .set_color(fg, bg)
                .map_err(|_| std::io::Error::other("Failed to set color"))?;

            self.output
                .write_str(cell.symbol())
                .map_err(|_| std::io::Error::other("Failed to write character"))?;
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        // Not supported on all platforms.
        let _ = self.output.enable_cursor(false);

        Ok(())
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        // Not supported on all platforms.
        let _ = self.output.enable_cursor(true);

        Ok(())
    }

    fn get_cursor_position(&mut self) -> std::io::Result<ratatui::prelude::Position> {
        let (col, row) = self.output.cursor_position();

        Ok(ratatui::prelude::Position {
            x: col as u16,
            y: row as u16,
        })
    }

    fn set_cursor_position<P: Into<ratatui::prelude::Position>>(
        &mut self,
        position: P,
    ) -> std::io::Result<()> {
        let pos = position.into();

        self.output
            .set_cursor_position(pos.x as usize, pos.y as usize)
            .map_err(|_| std::io::Error::other("Failed to set cursor position"))
    }

    fn clear(&mut self) -> std::io::Result<()> {
        self.output
            .clear()
            .map_err(|_| std::io::Error::other("Failed to clear"))
    }

    fn size(&self) -> std::io::Result<ratatui::prelude::Size> {
        let mode = self
            .output
            .current_mode()
            .map_err(|_| std::io::Error::other("Failed to get current mode"))?;

        let mode = mode.ok_or_else(|| std::io::Error::other("No current mode available"))?;

        Ok(ratatui::prelude::Size {
            width: mode.columns() as u16,
            height: (mode.rows() - 2) as u16,
        })
    }

    fn window_size(&mut self) -> std::io::Result<ratatui::backend::WindowSize> {
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

    fn flush(&mut self) -> std::io::Result<()> {
        // No-op?
        Ok(())
    }
}
