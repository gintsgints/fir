use cli_log::{init_cli_log, warn, log_mem, Level, info};

fn main() {
    init_cli_log!();

    match fir::cli::run() {
        Ok(()) => {}
        Err(e) => {
            warn!("Error: {}", e);
            eprintln!("{e}");
        }
    }

    log_mem(Level::Info);
    info!("bye");
}
