use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;

use rip_tower::app::App;
use rip_tower::start_ui;

// main.rs
fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new())); // TODO app is useless for now
    start_ui(app)?;
    Ok(())
}
