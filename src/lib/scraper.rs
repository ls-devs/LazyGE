use chrono::{Datelike, Utc};
use std::io::{self, Write};
use std::string::String;
use std::time::Duration;
use thirtyfour::prelude::*;

pub struct Scraper<'s> {
    pub url: &'s str,
    pub driver: WebDriver,
}

impl<'is> Scraper<'is> {
    // TODO: Ask for credentials, so we can go full headless
    pub async fn is_connected(&self, url: &str) -> WebDriverResult<bool> {
        self.driver.goto(url).await.unwrap();
        loop {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line");

            buffer = buffer.trim().to_string();

            if buffer.len().ge(&0) {
                break Ok(true);
            }

            break Ok(false);
        }
    }

    // TODO: This function should be a loop, so we can track real-time stats
    pub async fn get_stats(&self, _url: &str) -> WebDriverResult<()> {
        let driver = &self.driver;

        // Wait for DOM to be fully loaded
        driver
            .set_implicit_wait_timeout(Duration::new(11, 0))
            .await?;

        // Date is needed to see actual month stats
        let year = Utc::now().year();
        let mut month = Utc::now().month().to_string();
        let mut day = Utc::now().day().to_string();

        // Formatting Date to add 0 before date day/month number if <= 9
        // Global Exam use XX/XX/XXXX notation
        match month.parse::<u32>().unwrap() <= 9 {
            true => {
                month = format!("0{}", month);
            }
            false => (),
        }
        if day.parse::<u32>().unwrap() <= 9 {
            day = format!("0{}", day);
        }

        driver.goto("https://exam.global-exam.com/stats").await?;

        // Wait for block "Total Activities" to be fully loaded
        driver
            .query(By::XPath(
                "/html/body/div[1]/main/div[2]/div/div[2]/div[2]/div[3]/p",
            ))
            .first()
            .await?
            .wait_until()
            .error("No activities")
            .displayed()
            .await?;

        // Click on button to show DateForm picker
        driver
            .query(By::XPath(
                "/html/body/div[1]/main/div[2]/div/div[2]/div[2]/div[3]/div/div/div/div[1]/div",
            ))
            .first()
            .await?
            .click()
            .await?;

        // Clear Actual DateFormInput
        driver.query(By::XPath("/html/body/div[1]/main/div[2]/div/div[2]/div[2]/div[3]/div/div/div[2]/div[2]/div[1]/input[1]"))
        .first().await?.clear().await?;

        // Fill Actual DateFormInput with Date from the beginning fo the month
        driver.query(By::XPath("/html/body/div[1]/main/div[2]/div/div[2]/div[2]/div[3]/div/div/div[2]/div[2]/div[1]/input[1]"))
        .first().await?
        .send_keys(format!("{}{}{}", "01", month, year)).await?;

        // Clear Until DateFormInput
        driver.query(By::XPath("/html/body/div[1]/main/div[2]/div/div[2]/div[2]/div[3]/div/div/div[2]/div[2]/div[1]/input[2]"))
        .first().await?.clear().await?;

        // Fill Until DateFormInput with actual Date
        driver.query(By::XPath("/html/body/div[1]/main/div[2]/div/div[2]/div[2]/div[3]/div/div/div[2]/div[2]/div[1]/input[2]"))
        .first().await?
        .send_keys(format!("{}{}{}", day, month, year)).await?;

        // Click on Confirm Button to show stats
        driver.query(By::XPath("//*[@id='stats-content']/div[2]/div[2]/div[3]/div/div/div[2]/div[2]/div[2]/button[2]")).first().await?.click().await?;

        // Grab total time of the month
        let mut total_time = driver
            .query(By::XPath(
                "//*[@id='stats-content']/div[2]/div[2]/div[3]/div/div/div/div[2]/div[1]/p[2]",
            ))
            .first()
            .await?
            .text()
            .await?;

        let mut total_activties = driver
            .query(By::XPath(
                "//*[@id='stats-content']/div[2]/div[2]/div[3]/div/div/div/div[2]/div[2]/p[2]",
            ))
            .first()
            .await?
            .text()
            .await?;

        if total_activties == "-" {
            total_activties = "0".to_string();
        }

        if total_time == "-" {
            total_time = "0".to_string();
        }

        // let mut stats: HashMap<&str, &String> = HashMap::new();
        // stats.insert("Total time ", &total_time);
        // stats.insert("Total activities ", &total_activties);

        print!("#############################\n");
        print!("## Total time :       {}   ##\n", total_time);
        print!("## Total activities : {}   ##\n", total_activties);
        print!("#############################\n");
        io::stdout().flush().unwrap();

        self.check_exercises().await?;

        Ok(())
    }

    pub async fn check_exercises(&self) -> WebDriverResult<()> {
        let driver = &self.driver;

        // Go to plannings
        driver
            .goto("https://exam.global-exam.com/user-plannings")
            .await?;

        // Wait for planning to be loaded
        driver
            .query(By::XPath("/html/body/div[1]/div/main/div/a"))
            .first()
            .await?
            .wait_until()
            .error("No planning")
            .displayed()
            .await?;

        // Click on planning to go to the exercises
        driver
            .query(By::XPath("/html/body/div[1]/div/main/div/a"))
            .first()
            .await?
            .click()
            .await?;

        // Wait for the page to be loaded
        driver
            .query(By::XPath("/html/body/div[1]/main/div[2]/div/div/div[2]/h1"))
            .first()
            .await?
            .wait_until()
            .error("No heading")
            .displayed()
            .await?;

        // Grab the main container
        let exercices_container = driver
            .query(By::XPath("/html/body/div[1]/main/div[2]/div/div/div[2]"))
            .first()
            .await?;

        // Grab each exercises blocks
        let exercices_blocks = exercices_container
            .query(By::ClassName("grid"))
            .all()
            .await?;

        // Loop on blocks
        for blocks in exercices_blocks {
            // Grab all exercises buttons
            let buttons = blocks.find_all(By::Tag("button")).await?;
            // Loop on the buttons
            for exercise in buttons {
                // Grab the span container
                let first_span = exercise.query(By::Tag("span")).first().await?;
                // Check if the image is Listening or Reading
                let is_listening_or_reading = first_span
                    .query(By::Tag("img"))
                    .first()
                    .await?
                    .prop("src")
                    .await?;

                // Sort exercises
                match is_listening_or_reading.as_deref() {
                    // Is reading ?
                     Some("https://content.globalexam.cloud/exam/assets/img/content/sections/reading.png") => {
                        println!("Reading exercise");
                        let is_done_elem = first_span
                        .query(By::ClassName("rounded-full"))
                        .first()
                        .await;
                        match is_done_elem {
                            Ok(_elem) => {
                                println!("Exercice done");
                            }
                            Err(_e) => {
                                // Exercice not done
                                println!("Not done");
                            }
                        }

                    },

                    // Is listenning ?
                    Some("https://content.globalexam.cloud/exam/assets/img/content/sections/listening.png") => {
                        println!("Listening exercise");
                        let is_done_elem = first_span
                        .query(By::ClassName("rounded-full"))
                        .first()
                        .await;
                        match is_done_elem {
                            Ok(_elem) => {
                                println!("Exercice done");
                            }
                            Err(_e) => {
                                // Exercice not done
                                println!("Not done");
                            }
                        }

                    },
                    // If this is not reading or not listening, just dont do the exercise
                    Some(_) => println!("Not reading and not listenning, dont do this exercise"),
                    // No element found => Err
                    None => panic!("Not value found"),
                
                };

            }
        }

        Ok(())
    }
}
