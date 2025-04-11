use crate::Result;

use ratatui::{
    Frame, Terminal,
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, List, ListState, Paragraph},
};

use crate::{UefiInputReader, UefiOutputBackend};

pub(crate) struct Ui {
    terminal: Terminal<UefiOutputBackend>,
    input_reader: UefiInputReader,
    state: UiState,
}

impl Ui {
    pub fn new(terminal: Terminal<UefiOutputBackend>, input_reader: UefiInputReader) -> Self {
        Self {
            terminal,
            input_reader,
            state: UiState {
                status_text: "Welcome to bootox!".to_string(),
                ..Default::default()
            },
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        self.terminal.clear()?;
        Ok(())
    }

    pub fn wait_for_keypress(&mut self) {
        let _ = self.input_reader.read_event();
    }

    pub fn greet(&mut self) -> Result<()> {
        // Clear the screen.
        self.clear()?;

        // Greet!
        self.update_status("Welcome! Press ENTER to start.")?;

        // Wait for input.
        loop {
            if let Some(event) = self.input_reader.read_event()? {
                match event {
                    terminput::Event::Key(terminput::KeyEvent {
                        code: terminput::KeyCode::Enter,
                        ..
                    }) => break,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn update_status<S: AsRef<str>>(&mut self, status: S) -> Result<()> {
        self.state.status_text = status.as_ref().to_string();
        self.flush()?;

        Ok(())
    }

    pub fn show_menu(&mut self, options: &[&str]) -> Result<usize> {
        self.state.show_menu = true;
        self.state.menu_items = options.iter().map(|s| (*s).to_owned()).collect();
        self.state.menu_state.select_first();
        self.flush()?;

        loop {
            if let Some(event) = self.input_reader.read_event()? {
                match event {
                    terminput::Event::Key(terminput::KeyEvent {
                        code: terminput::KeyCode::Up,
                        ..
                    }) => {
                        self.state.menu_state.select_previous();
                        self.flush()?;
                    }
                    terminput::Event::Key(terminput::KeyEvent {
                        code: terminput::KeyCode::Down,
                        ..
                    }) => {
                        self.state.menu_state.select_next();
                        self.flush()?;
                    }
                    terminput::Event::Key(terminput::KeyEvent {
                        code: terminput::KeyCode::Enter,
                        ..
                    }) => break,
                    _ => {}
                }
            }

            self.flush()?;
        }

        self.state.show_menu = false;
        self.flush()?;

        Ok(self.state.menu_state.selected().unwrap_or(0))
    }

    pub fn output_line<S: AsRef<str>>(&mut self, line: S) -> Result<()> {
        self.state.append_output(line.as_ref());
        self.state.append_output("\r\n");
        self.flush()?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.terminal.draw(|frame| self.state.render(frame))?;
        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct UiState {
    status_text: String,
    output: String,
    show_menu: bool,
    menu_items: Vec<String>,
    menu_state: ListState,
}

impl UiState {
    pub fn render(&mut self, frame: &mut Frame) {
        // TODO: Investigate why this fails.
        // let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]);
        // let chunks = layout.split(frame.area());

        // Manually compute the layout.
        let mut upper = frame.area();
        upper.height -= 3;
        let mut output_rect = upper;
        output_rect.x += upper.width / 2;
        output_rect.width = upper.width - output_rect.x;
        let mut list_rect = upper;
        list_rect.width = output_rect.x;
        let mut status_rect = frame.area();
        status_rect.height = 3;
        status_rect.y = upper.y + upper.height;

        let style = Style::new().cyan().italic();
        let status = Span::styled(self.status_text.as_str(), style);
        let status_bar = Paragraph::new(status)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .alignment(Alignment::Left);

        let main = Paragraph::new(self.output.as_str()).block(Block::bordered().title("bootox"));

        if self.show_menu {
            let list = List::new(self.menu_items.iter().map(|i| i.as_str()))
                .style((Color::Gray, Color::Black))
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> ")
                .block(Block::bordered().title("Menu"));

            frame.render_stateful_widget(list, list_rect, &mut self.menu_state);
        }

        frame.render_widget(main, output_rect);
        frame.render_widget(status_bar, status_rect);
    }

    pub fn append_output<S: AsRef<str>>(&mut self, line: S) {
        self.output.push_str(line.as_ref());
    }
}
