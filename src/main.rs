pub mod app;
pub mod input_widget;
pub mod string_ops;
pub mod tabs;
pub mod template_info;

use color_eyre::Result;

use crate::{app::App, template_info::fetch_template_info};

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        simple_logging::log_to_file(".logs", log::LevelFilter::Info)?;
    }
    color_eyre::install()?;
    let eyre_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        eyre_hook(panic_info);
        eprintln!("{}", panic_info);
    }));
    let terminal = ratatui::init();
    fetch_template_info().unwrap_or_else(|err| panic!("{err}"));
    let mut app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    if let Some(completion_cb) = app.on_complete {
        completion_cb();
    }
    result
}
