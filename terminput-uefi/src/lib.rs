use uefi::{
    ResultExt,
    boot::{self, ScopedProtocol},
    proto::console::{
        self,
        text::{Key, ScanCode},
    },
};

pub struct UefiInputReader {
    input: ScopedProtocol<console::text::Input>,
}

impl UefiInputReader {
    pub fn new(input: ScopedProtocol<console::text::Input>) -> Self {
        Self { input }
    }

    pub fn read_event(&mut self) -> uefi::Result<Option<terminput::Event>> {
        // Pause until a keyboard event occurs.
        let mut events = [self.input.wait_for_key_event().unwrap()];
        boot::wait_for_event(&mut events).discard_errdata()?;

        let event = self.input.read_key()?;
        let code = match event {
            Some(Key::Printable(c)) if c == '\r' => Some(terminput::KeyCode::Enter),
            Some(Key::Printable(c)) => Some(terminput::KeyCode::Char(c.into())),
            Some(Key::Special(ScanCode::ESCAPE)) => Some(terminput::KeyCode::Esc),
            Some(Key::Special(ScanCode::UP)) => Some(terminput::KeyCode::Up),
            Some(Key::Special(ScanCode::DOWN)) => Some(terminput::KeyCode::Down),
            Some(Key::Special(ScanCode::LEFT)) => Some(terminput::KeyCode::Left),
            Some(Key::Special(ScanCode::RIGHT)) => Some(terminput::KeyCode::Right),
            Some(Key::Special(ScanCode::PAGE_UP)) => Some(terminput::KeyCode::PageUp),
            Some(Key::Special(ScanCode::PAGE_DOWN)) => Some(terminput::KeyCode::PageDown),
            Some(Key::Special(ScanCode::HOME)) => Some(terminput::KeyCode::Home),
            Some(Key::Special(ScanCode::END)) => Some(terminput::KeyCode::End),
            Some(Key::Special(ScanCode::INSERT)) => Some(terminput::KeyCode::Insert),
            Some(Key::Special(ScanCode::DELETE)) => Some(terminput::KeyCode::Delete),
            Some(Key::Special(ScanCode::FUNCTION_1)) => Some(terminput::KeyCode::F(1)),
            Some(Key::Special(ScanCode::FUNCTION_2)) => Some(terminput::KeyCode::F(2)),
            Some(Key::Special(ScanCode::FUNCTION_3)) => Some(terminput::KeyCode::F(3)),
            Some(Key::Special(ScanCode::FUNCTION_4)) => Some(terminput::KeyCode::F(4)),
            Some(Key::Special(ScanCode::FUNCTION_5)) => Some(terminput::KeyCode::F(5)),
            Some(Key::Special(ScanCode::FUNCTION_6)) => Some(terminput::KeyCode::F(6)),
            Some(Key::Special(ScanCode::FUNCTION_7)) => Some(terminput::KeyCode::F(7)),
            Some(Key::Special(ScanCode::FUNCTION_8)) => Some(terminput::KeyCode::F(8)),
            Some(Key::Special(ScanCode::FUNCTION_9)) => Some(terminput::KeyCode::F(9)),
            Some(Key::Special(ScanCode::FUNCTION_10)) => Some(terminput::KeyCode::F(10)),
            Some(Key::Special(ScanCode::FUNCTION_11)) => Some(terminput::KeyCode::F(11)),
            Some(Key::Special(ScanCode::FUNCTION_12)) => Some(terminput::KeyCode::F(12)),
            Some(Key::Special(ScanCode::MUTE)) => Some(terminput::KeyCode::Media(
                terminput::MediaKeyCode::MuteVolume,
            )),
            Some(Key::Special(ScanCode::VOLUME_UP)) => Some(terminput::KeyCode::Media(
                terminput::MediaKeyCode::RaiseVolume,
            )),
            Some(Key::Special(ScanCode::VOLUME_DOWN)) => Some(terminput::KeyCode::Media(
                terminput::MediaKeyCode::LowerVolume,
            )),
            Some(Key::Special(_)) => None,
            None => None,
        };

        if let Some(code) = code {
            Ok(Some(terminput::Event::Key(terminput::KeyEvent {
                code,
                modifiers: terminput::KeyModifiers::empty(),
                kind: terminput::KeyEventKind::Press,
                state: terminput::KeyEventState::NONE,
            })))
        } else {
            Ok(None)
        }
    }
}
