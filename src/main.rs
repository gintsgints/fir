use cli_log::{info, init_cli_log, log_mem, Level};
use fir::{app::App, errors::ProgramError};

fn main() -> Result<(), ProgramError> {
    init_cli_log!();

    let mut application = App::new()?;

    application.run()?;

    log_mem(Level::Info);
    info!("bye");
    Ok(())
}
