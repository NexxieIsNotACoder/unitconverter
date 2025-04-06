// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{error::Error, result};
slint::include_modules!();

//check if string is either empty or doesn't have numbers
fn stringchecker(string: String) -> bool{
    if string.is_empty() == true { 
        return false;
    }
    for (x) in string.chars() {
        // if input string contains a decimal
        if x == '.' {
            // check if the input string has numbers
            if string.len() > 1{
                break; 
            }
            else {
                return false;
            }
}
        if x.is_numeric() == false{
            return false;
        }
    }

    return true;
}
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    ui.on_convertToF({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            if stringchecker(string.to_string()) == false {
                let result = format!("Please input a valid number!");
                ui.set_results(result.into());
            }
            else {
                let num: f64 = string.trim().parse().unwrap();
                // convert num to celcius
                let celcius: f64 = ((num-32.0)*5.0)/9.0;
                let result = format!("{:.2}°C", {celcius});
                ui.set_results(result.into());
            }
        }
    });
    ui.on_convertToKMH({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            if stringchecker(string.to_string()) == false  {
                let result = format!("Please input a valid number!");
                ui.set_results(result.into());
            }
            else{
                let num: f64 = string.trim().parse().unwrap();
                if num < 0.0 {
                    let result = format!("Please input a positive number!");
                    ui.set_results(result.into());
                }
                // convert num (miles) to km
                else {
                    let km: f64 = num*1.609347218694;
                    let result = format!("{:.2} Kilometers", {km});
                    ui.set_results(result.into());
                }
            }
        }
    });

    ui.run()?;

    Ok(())
}
