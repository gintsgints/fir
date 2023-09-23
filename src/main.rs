
fn main() {
    match fir::cli::run() {
        Ok(()) => {},
        Err(e) => {
            println!("Error: {}", e)
        },
    }
}
