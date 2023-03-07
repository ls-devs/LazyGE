use sysinfo::{ProcessExt, System, SystemExt};

use std::{
    process::{Command, Stdio},
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use nonblock::NonBlockingReader;
use thirtyfour::{prelude::WebDriverError, ChromeCapabilities, DesiredCapabilities, WebDriver};

static DRIVER_URL: &str = "http://localhost:9514";

pub struct Driver {}

impl Driver {
    /// # Start the chromedriver
    pub fn run_chromedriver(path: String) -> bool {
        // Create channel to communicate with the thread
        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();

        // Create a new thread
        let th = thread::spawn(move || {
            // If driver already running somewhere kill it
            let s = System::new_all();
            for (_pid, process) in s.processes() {
                if process.name().contains("chromedriver") {
                    process.kill();
                    thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                }
            }

            // Start chromedriver
            let mut child = Command::new(path)
                .args(&["--ip=localhost", "--port=9514", "--log-level=OFF"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            // Read driver stdout with non blocking BufReader
            let stdout = child.stdout.take().unwrap();
            let mut noblock_stdout = NonBlockingReader::from_fd(stdout).unwrap();
            thread::sleep(Duration::from_secs(5));
            while !noblock_stdout.is_eof() {
                let mut buf = String::new();
                noblock_stdout.read_available_to_string(&mut buf).unwrap();
                std::thread::sleep(Duration::from_secs(1));
                // If driver spawn successfully, return
                if buf.contains("ChromeDriver was started successfully.") {
                    tx.send(true).unwrap();
                    break;
                } else {
                    tx.send(false).unwrap();
                    break;
                }
            }
        });

        // Loop until thread as finished
        loop {
            if th.is_finished() {
                break;
            }
        }

        rx.recv().unwrap()
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
        if Self::run_chromedriver(path) {
            let driver = WebDriver::new(DRIVER_URL, Self::set_capabilities().unwrap())
                .await
                .unwrap();
            driver.set_window_rect(0, 0, 10, 10).await.unwrap();
            driver
        } else {
            panic!("not ok");
        }
    }

    pub fn kill_driver(&self) {
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
