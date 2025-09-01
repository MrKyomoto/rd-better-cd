mod app;
mod components;
mod ui;
use std::error::Error;

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    if app.output {
        println!("{}", app.finnal_dir());
    } else {
        println!("{}", app.current_dir);
    }
    app_result
}
