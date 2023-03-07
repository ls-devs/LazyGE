use std::io;
use thirtyfour::prelude::*;

pub struct Scraper<'s> {
    pub url: &'s str,
    pub driver: WebDriver,
}

impl<'is> Scraper<'is> {
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

    pub async fn get_ge_stats(&self, _url: &str) -> WebDriverResult<()> {
        self.driver.set_window_rect(0, 0, 0, 0).await.unwrap();
        // let driver = Self::driver(&self);
        // driver.goto(url).await.unwrap();
        // driver.clone().quit().await.unwrap();
        Ok(())
    }

    // pub fn driver(&self) -> &WebDriver {
    // &self.driver
    // }
}
