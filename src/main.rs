// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

// Todo: Add formulas for ounces -> grams, lbs -> kgs, and gallons -> litres conversion
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;
slint::include_modules!();
const MITOKFACTOR: f64 = 1.609347218694;
const OUNCESTOGRAMSFACTOR: f64 = 28.3495;
const LBSTOKGFACTOR: f64 = 2.2046;
const GALLONSTOLITERSFACTOR: f64 = 3.785412;
//check if string is either empty or doesn't have numbers
fn stringchecker(string: String) -> bool{
    if string.is_empty() { 
        return false;
    }
    for x in string.chars() {
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
        if !x.is_numeric() {
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
                // check if number is positive
                if num < 0.0 {
                    let result = format!("Please input a positive number!");
                    ui.set_results(result.into());
                }
                // convert num (miles) to km
                else {
                    let km: f64 = num*MITOKFACTOR;
                    let result = format!("{:.2} Kilometers", {km});
                    ui.set_results(result.into());
                }
            }
        }
    });
    ui.on_convertToGrams({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            if stringchecker(string.to_string()) == false  {
                let result = format!("Please input a valid number!");
                ui.set_results(result.into());
            }
            else{
                let num: f64 = string.trim().parse().unwrap();
                // check if number is positive
                if num < 0.0 {
                    let result = format!("Please input a positive number!");
                    ui.set_results(result.into());
                }
                // convert num (ounces) to grams
                else {
                    let grams: f64 = num*OUNCESTOGRAMSFACTOR;
                    let result = format!("{:.2} Grams", {grams});
                    ui.set_results(result.into());
                }
            }
        }
    });
    ui.on_convertToKGs({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            if stringchecker(string.to_string()) == false  {
                let result = format!("Please input a valid number!");
                ui.set_results(result.into());
            }
            else{
                let num: f64 = string.trim().parse().unwrap();
                // check if number is positive
                if num < 0.0 {
                    let result = format!("Please input a positive number!");
                    ui.set_results(result.into());
                }
                // convert num (lbs) to kgs
                else {
                    let kg: f64 = num/LBSTOKGFACTOR;
                    let result = format!("{:.2} Kilograms", {kg});
                    ui.set_results(result.into());
                }
            }
        }
    });
    ui.on_convertToL({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            if stringchecker(string.to_string()) == false  {
                let result = format!("Please input a valid number!");
                ui.set_results(result.into());
            }
            else{
                let num: f64 = string.trim().parse().unwrap();
                // check if number is positive
                if num < 0.0 {
                    let result = format!("Please input a positive number!");
                    ui.set_results(result.into());
                }
                // convert num (gallons) to liters
                else {
                    let liters: f64 = num*GALLONSTOLITERSFACTOR;
                    let result = format!("{:.2} Liters", {liters});
                    ui.set_results(result.into());
                }
            }
        }
    });
    ui.run()?;

    Ok(())
}
