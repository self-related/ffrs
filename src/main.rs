pub mod utils;
mod init;

fn main() {
    let result = init::init();
    if result.is_err() {
        println!("\n\nERROR: {}", result.unwrap_err());
    }

    println!("\n\n\npress Enter\n");
    _ = utils::readln_trimmed();
}
