use std::io::{self, stdout};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    // 下划线变量名会让编译器忽略这个变量没有使用的警告
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?}\r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('d') => break,
                _ => println!("{:?}\r", key),
            },
            Err(err) => die(err),
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
