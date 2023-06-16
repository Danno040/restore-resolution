use std::ptr;

use core_graphics::{display::{CGDisplay, CGDisplayMode, CGConfigureOption}, base::CGError};

fn print_error_type(error: CGError) {
    match error {
        1000 => println!("A general failure occurred.: https://developer.apple.com/documentation/coregraphics/cgerror/failure"),
        1001 => println!("One or more of the parameters passed to a function are invalid. Check for NULL pointers.: https://developer.apple.com/documentation/coregraphics/cgerror/illegalargument"),
        1004 => println!("The requested operation is inappropriate for the parameters passed in, or the current system state.: https://developer.apple.com/documentation/coregraphics/cgerror/cannotcomplete"),
        1002 => println!("The parameter representing a connection to the window server is invalid."),
        1003 => println!("The CPSProcessSerNum or context identifier parameter is not valid."),
        1010 => println!("The requested operation is not valid for the parameters passed in, or the current system state."),
        1011 => println!("The requested operation could not be completed as the indicated resources were not found."),
        1006 => println!("Return value from obsolete function stubs present for binary compatibility, but not typically called."),
        1007 => println!("A parameter passed in has a value that is inappropriate, or which does not map to a useful operation or value."),
        1008 => println!("A data type or token was encountered that did not match the expected type or token."),
        _ => println!("Unknown error"),
    }
}

fn main() {
    println!("Hello, world!");

    let active_displays_result = CGDisplay::active_displays();
    if active_displays_result.is_err() {
        let e = active_displays_result.unwrap_err();
        println!("Error Getting Values: {e:?}");
        return;
    }

    let active_display_ids = active_displays_result.unwrap();
    if active_display_ids.len() != 2 {
        println!("Not enough displays. Got {}, not 2. So dying here.", active_display_ids.len());
        return;
    }

    for display_id in active_display_ids.iter() {
        let display = CGDisplay::new(*display_id);

        if display.serial_number() != 959853388u32 {
            println!("Found non-home display. Display ID = {}, serial number = {}", display_id, display.serial_number());
            continue;
        }

        // Verify is main display:
        if !display.is_main() {
            println!("Home display is NOT main. This will have to be modified in the System Preferences directly.");
        }

        let display_result = display.display_mode();
        if display_result.is_none() {
            println!("Couldn't fetch display mode for display id {}, serial number = {}", display_id, display.serial_number());
            continue;
        }
        let display_mode = display_result.unwrap();

        // Check if it's set to 1920x1080x32@60:
        if display_mode.width() == 1920u64 && display_mode.height() == 1080u64 && display_mode.bit_depth() == 32usize && display_mode.refresh_rate() == 60f64 {
            println!("Mode set correctly. No actions needed.");
            return;
        }

        let all_display_modes_option = CGDisplayMode::all_display_modes(*display_id, ptr::null());
        if all_display_modes_option.is_none() {
            println!("Couldn't list display modes. :(");
            return;
        }
        let all_display_modes = all_display_modes_option.unwrap();
        let mut desired_display_mode = None;
        for possible_mode in all_display_modes.iter() {
            if possible_mode.width() == 1920u64 && possible_mode.height() == 1080u64 && possible_mode.bit_depth() == 32usize && possible_mode.refresh_rate() == 60f64 {
                println!("Found desired display mode. {}", possible_mode.mode_id());
                desired_display_mode = Some(possible_mode.clone());
                break;
            }
        }
        if desired_display_mode.is_none() {
            println!("Couldn't find correct display mode.");
            return;
        }

        println!("Display mode is incorrect ({}x{}x{}@{}). Resetting to 1920x1080x32@60.", display_mode.width(), display_mode.height(), display_mode.bit_depth(), display_mode.refresh_rate());
        let config_ref_result = display.begin_configuration();
        if config_ref_result.is_err() {
            println!("Failed to being config");
            return;
        }
        let config_ref = config_ref_result.unwrap();
        let mut configure_result = display.configure_display_with_display_mode(&config_ref, &desired_display_mode.unwrap());
        if configure_result.is_err() {
            println!("Failed to configure mode");
            return;
        }
        configure_result = display.complete_configuration(&config_ref, CGConfigureOption::ConfigurePermanently);
        if configure_result.is_err() {
            println!("Failed to complete:");
            print_error_type(configure_result.unwrap_err());
            return;
        }
    }
}
