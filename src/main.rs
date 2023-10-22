use cli_log::{info, init_cli_log, log_mem, Level};
use fir::app::App;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    init_cli_log!();

    let mut application = App::new()?;

    application.run()?;

    log_mem(Level::Info);
    info!("bye");
    Ok(())
}
