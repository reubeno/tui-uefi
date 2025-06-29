#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use uefi::{Status, entry, println, proto::console};

/// Performs the necessary setup code for the `uefi` crate.
fn setup_uefi_crate() {
    uefi::helpers::init().expect("Failed to initialize utilities");
}

fn create_ui() -> Result<(
    Terminal<ratatui_uefi::UefiOutputBackend>,
    terminput_uefi::UefiInputReader,
)> {
    let output_handle = uefi::boot::get_handle_for_protocol::<console::text::Output>()?;
    let output = uefi::boot::open_protocol_exclusive::<console::text::Output>(output_handle)?;

    let input_handle = uefi::boot::get_handle_for_protocol::<console::text::Input>()?;
    let input = uefi::boot::open_protocol_exclusive::<console::text::Input>(input_handle)?;

    let output_backend = ratatui_uefi::UefiOutputBackend::new(output);
    let terminal = Terminal::new(output_backend)?;

    let input_reader = terminput_uefi::UefiInputReader::new(input);

    Ok((terminal, input_reader))
}

struct App {
    style: Style,
}

impl App {
    fn new() -> Self {
        App {
            style: Style::default().bg(Color::Black).fg(Color::White),
        }
    }

    fn rotate_styles(&mut self) {
        self.style.fg = Some(match self.style.fg {
            Some(Color::White) => Color::Red,
            Some(Color::Red) => Color::Blue,
            Some(Color::Blue) => Color::Green,
            Some(Color::Green) => Color::Yellow,
            _ => Color::White,
        });
    }

    fn render(&self, frame: &mut Frame) {
        let lines = vec![
            Line::from("Hello, UEFI!").style(self.style),
            Line::from("Press 'q' to exit."),
            Line::from("Press DOWN to do something interesting!"),
        ];

        let status = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .centered();

        frame.render_widget(status, frame.area());
    }
}

fn run() -> Result<()> {
    // Instantiate our objects.
    let (mut terminal, mut input_reader) = create_ui()?;
    let mut app = App::new();

    terminal.clear()?;

    // Show the UI.
    loop {
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        // Wait for a keypress.
        if let Some(event) = input_reader.read_event()? {
            match event {
                terminput::Event::Key(terminput::KeyEvent {
                    code: terminput::KeyCode::Char('q'),
                    ..
                }) => break,
                terminput::Event::Key(terminput::KeyEvent {
                    code: terminput::KeyCode::Down,
                    ..
                }) => {
                    app.rotate_styles();
                }
                _ => {}
            }
        }
    }

    Ok(())
}

#[entry]
fn efi_main() -> Status {
    setup_uefi_crate();

    match run() {
        Ok(()) => {
            println!("Done.");
            Status::SUCCESS
        }
        Err(e) => {
            println!("!!! error: {:?}", e);
            Status::ABORTED
        }
    }
}
