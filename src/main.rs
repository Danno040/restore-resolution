use std::ptr;

use core_graphics::display::{CGDisplay, CGDisplayMode, CGConfigureOption};

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
                println!("Found desired display mode.");
                desired_display_mode = Some(possible_mode.clone());
            }
        }
        if desired_display_mode.is_none() {
            println!("Couldn't find correct display mode.");
            return;
        }

        println!("Display mode is incorrect. Resetting to 1920x1080x32@60.");
        let config_ref_result = display.begin_configuration();
        if config_ref_result.is_err() {
            return;
        }
        let config_ref = config_ref_result.unwrap();
        let mut configure_result = display.configure_display_with_display_mode(&config_ref, &desired_display_mode.unwrap());
        if configure_result.is_err() {
            return;
        }
        configure_result = display.complete_configuration(&config_ref, CGConfigureOption::ConfigurePermanently);
        if configure_result.is_err() {
            return;
        }
    }
}
