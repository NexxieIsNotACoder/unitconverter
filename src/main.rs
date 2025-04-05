// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    ui.on_convertToF({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let celcius: f64 = ((num-32.0)*5.0)/9.0;
            let result = format!("{:.2}°C", {celcius});
            ui.set_results(result.into());
        }
    });
    ui.on_convertToKMH({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let km: f64 = num*1.609347218694;
            let result = format!("{:.2} Kilometers", {km});
            ui.set_results(result.into());
        }
    });

    ui.run()?;

    Ok(())
}
