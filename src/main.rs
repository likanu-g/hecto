use std::io::{self, stdout, Read};

use termion::raw::IntoRawMode;

fn main() {
    // 下划线变量名会让编译器忽略这个变量没有使用的警告
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b)
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl_byte('d') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    println!("b2:{:#b}", byte & 0b0001_1111);
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
