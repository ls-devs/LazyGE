#![feature(async_closure)]
use catppuccin::Flavour;
use colored::*;
use std::{
    io::{self, stdin, Write},
    process,
};
#[path = "./lib/scraper.rs"]
mod scraper;
use scraper::Scraper;

#[path = "./lib/driver.rs"]
mod driver;
use driver::Driver;

#[path = "./lib/helper.rs"]
mod helper;
use helper::Utils;

#[tokio::main]
async fn main() {
    // Create a Helper
    let utils: Utils = Utils {};

    // Verify if chrome is in path
    match utils.is_in_path("chromium") {
        // Chrome is in path
        true => {
            let mut cond = false;
            let (r, g, b) = Flavour::Mocha.green().into();
            println!(
                "{}",
                "Enter GlobalExam URL given by your school :"
                    .custom_color(CustomColor::new(r, g, b))
            );
            while !cond {
                // Reading the URL from user input
                // Create a buffer for storing user_input
                let mut buffer = String::new();

                // Reading user input
                stdin().read_line(&mut buffer).unwrap();

                // Storing response
                let res: &str = buffer.trim_end();

                // Validate URL with a Regex
                match utils.regex_helper(&res) {
                    true => {
                        // Get the chrome version & OS
                        // Needed to run the correct driver
                        let version = utils.check_chrome_version();
                        let os = utils.check_os();

                        // Create the driver
                        let driver = Driver::create_driver(String::from(format!(
                            "/home/ls-devs/Projects/Perso/LazyGE/driver/{}/chromedriver-{}",
                            os, version
                        )))
                        .await;

                        // Create the Scraper
                        let scraper: Scraper = Scraper { url: &res, driver };

                        // Ask user to connect to GloabelExam
                        let (r, g, b) = Flavour::Mocha.blue().into();
                        println!(
                            "{}",
                            "Please connect to the plateforme and press ENTER"
                                .custom_color(CustomColor::new(r, g, b))
                        );
                        let is_connected = scraper.is_connected(res).await.unwrap();

                        if is_connected {
                            scraper.get_stats(&res).await.unwrap();
                            io::stdout().flush().unwrap();
                        }

                        cond = true;
                    }
                    false => {
                        let (r, g, b) = Flavour::Mocha.red().into();
                        print!(
                            "{}",
                            "Please, enter a valid URL\n".custom_color(CustomColor::new(r, g, b))
                        );
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }

        // Chrome not in path !
        false => {
            print!("Chrome not found, please intall to use this program");
            process::exit(1)
        }
    }
}
