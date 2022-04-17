use rustea::crossterm_event::{KeyCode, KeyEvent, KeyModifiers};
use rustea::{quit_cmd, App, Command, Message};

struct Model {
    last_key: Option<char>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(quit_cmd)),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Char(c) => {
                    self.last_key = Some(c);
                    return None;
                }
                _ => unimplemented!(),
            }
        };

        None
    }

    fn view(&self) -> String {
        format!("Hello! You pressed: {:?}", self.last_key)
    }
}

fn main() {
    let model = Model { last_key: None };

    rustea::run(model).unwrap();
}
