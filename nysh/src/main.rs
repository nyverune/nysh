mod checks;

use std::io;
use std::process::Command;
use std::str::FromStr;
use strum_macros::EnumString;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use checks::is_program_installed;

#[derive(Debug, Default)]
pub struct App {
    ssh_identities_count: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Nysh ".bold());
        let instructions = Line::from(vec![
            " New ".into(),
            "[n]".blue().bold(),
            " Quit ".into(),
            " [q] ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let hello_world = Text::from(vec![Line::from(vec![
            "Identities count: ".into(),
            self.ssh_identities_count.to_string().yellow(),
        ])]);

        Paragraph::new(hello_world)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[derive(Debug, PartialEq, EnumString, strum::Display)]
enum Algorithm {
    #[strum(serialize = "ecdsa")]
    Ecdsa,

    #[strum(serialize = "ecdsa-sk")]
    EcdsaSk,

    #[strum(serialize = "ed25519")]
    Ed25519,

    #[strum(serialize = "ed25519-sk")]
    Ed25519Sk,

    #[strum(serialize = "mldsa44-ed25519")]
    Mldsa44Ed25519,

    #[strum(serialize = "rsa")]
    Rsa,
}

struct GenerateKeyProps {
    alias: String,
    email: String,
    path: String,
    algorithm: Algorithm,
    passphrasse: Option<String>,
    bits: u32,
}

fn generate_key(props: GenerateKeyProps) {
    Command::new("ssh-keygen")
        .args(["-t", &props.algorithm.to_string(), "-C", &props.email])
        .status()
        .expect("ssh-keygen failed");
}

// fn main() -> io::Result<()> {
fn main() {
    // ratatui::run(|terminal| App::default().run(terminal))
    let is_installed = is_program_installed("ssh-agent");
    println!("{}", is_installed);
}
