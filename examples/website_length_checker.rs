use rustea::{
    component::input::Input,
    crossterm_event::{KeyCode, KeyEvent},
    App, Command, Message,
};

struct Model {
    url_input: Input,
    website_length: Option<usize>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            match key_event.code {
                KeyCode::Enter => {
                    let url = self.url_input.buffer();
                    self.url_input.clear();

                    return Some(make_request_command(&url));
                }
                _ => self.url_input.on_key_event(*key_event),
            }
        } else if let Some(len) = msg.downcast_ref::<WebsiteLengthMessage>() {
            self.website_length = Some(len.0);
        }

        None
    }

    fn view(&self) -> String {
        let mut out = format!(
            "Website URL (press enter when done): {}",
            self.url_input.buffer()
        );
        if let Some(len) = self.website_length {
            out.push_str(&format!("\nWebsite length: {}", len));
        }

        out
    }
}

struct WebsiteLengthMessage(usize);

fn make_request_command(url: &str) -> Command {
    // It's okay to block since commands are multi threaded
    let website_len = reqwest::blocking::get(url).unwrap().bytes().unwrap().len();
    Box::new(move || Some(Box::new(WebsiteLengthMessage(website_len))))
}

fn main() {
    rustea::run(Model {
        url_input: Input::new(),
        website_length: None,
    })
    .unwrap();
}