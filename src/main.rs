mod app;
mod components;
mod ui;
use std::error::Error;
use std::fs::{self};

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new()?;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    let temp_path = "/tmp/rd_selected_dir";
    if app.output {
        fs::write(temp_path, app.finnal_dir()?)?;
    } else {
        fs::write(temp_path, app.current_dir)?;
    }
    app_result
}
