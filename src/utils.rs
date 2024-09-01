use std::{io::stdin, thread, time::Duration};

pub fn wait_milis(milliseconds: usize) {
    thread::sleep(Duration::from_millis(milliseconds as u64));
}

pub fn readln_trimmed() -> String {
    let mut buffer: String = String::new();
    _ = stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}
