use chrono::{Datelike, Local, Timelike};
use system_shutdown::shutdown;
use wifiscanner;

fn main() {
    loop {
        if is_between_friday_sunday_weekend() {
            println!("{:?}", wifiscanner::scan());

            let interfaces = wifiscanner::scan().unwrap();

            for interface in interfaces {
                if interface.ssid == "kue_ssid" {
                    println!("Found the network! {}", interface.ssid);
                    // shutdown the computer
                    match shutdown() {
                        Ok(_) => println!("Shutting down, bye!"),
                        Err(error) => eprintln!("Failed to shut down: {}", error),
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(600));
    }
}

fn is_between_friday_sunday_weekend() -> bool {
    let now = Local::now();
    let day_of_week = now.weekday();
    let hour = now.hour();
    if day_of_week == chrono::Weekday::Fri && (hour > 18 || (hour == 18)) {
        return true;
    }

    if day_of_week == chrono::Weekday::Sat || day_of_week == chrono::Weekday::Sun {
        return true;
    }

    if day_of_week == chrono::Weekday::Sun && (hour < 18 || (hour == 18)) {
        return true;
    }

    false
}
