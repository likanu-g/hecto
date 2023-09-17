use crate::Terminal;

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refush_screen() {
                die(&error)
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error)
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn refush_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Good bye. \r")
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0); //清屏后光标移动到终端(1,1)的坐标
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('d') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hello editor -- version {} \r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let space = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", space, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
