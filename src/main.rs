use std::io::{stdout, Result};

use crossterm::{ExecutableCommand, style::{SetForegroundColor, SetBackgroundColor, ResetColor, Print, Color}};

fn main() -> Result<()> {
    println!("Hello, world!");

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;
    Ok(())
}
