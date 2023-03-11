use std::{process::Stdio, thread};

use sysinfo::{ProcessExt, System, SystemExt};

use thirtyfour::{prelude::WebDriverError, ChromeCapabilities, DesiredCapabilities, WebDriver};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

static DRIVER_URL: &str = "http://localhost:9514";

pub struct Driver {}

impl Driver {
    /// # Start the chromedriver
    pub async fn run_chromedriver(path: String) -> bool {
        // If driver already running somewhere kill it
        Self::kill_driver();

        // Start the chromedriver
        let mut chromedriver = Command::new(path)
            .args(&["--ip=localhost", "--port=9514", "--log-level=OFF"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("!");

        let mut buffer = BufReader::new(chromedriver.stdout.take().unwrap()).lines();
        let mut cond: bool = false;

        // Read process output so we can check the driver status
        while let Some(line) = buffer.next_line().await.unwrap() {
            if line.contains("ChromeDriver was started successfully.") {
                cond = true;
                break;
            } else {
                println!("Cannot start chrome driver, exiting");
            }
        }

        cond
    }

    /// Sets the capabilities of the WebDriver.
    /// Config : headless, incogniton, disable_dev_shm, no_sandbox
    pub fn set_capabilities() -> Result<ChromeCapabilities, WebDriverError> {
        let mut caps = DesiredCapabilities::chrome();
        caps.set_no_sandbox()?;
        caps.set_disable_dev_shm_usage()?;
        caps.add_chrome_arg("--incognito")?;
        // caps.set_headless()?;
        Ok(caps)
    }

    // Create the WebDriver and connect to the chromedriver
    pub async fn create_driver(path: String) -> WebDriver {
        if Self::run_chromedriver(path).await {
            let driver = WebDriver::new(DRIVER_URL, Self::set_capabilities().unwrap())
                .await
                .unwrap();
            driver
        } else {
            panic!("not ok");
        }
    }

    // # Kill the chromedriver
    pub fn kill_driver() {
        let s = System::new_all();
        for (_pid, process) in s.processes() {
            if process.name().contains("chromedriver") {
                process.kill();
                thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        }
    }
}
