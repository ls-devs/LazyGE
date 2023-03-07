use lazy_static::lazy_static;
use regex::Regex;
use std::{
    env,
    fmt::{self, Display},
    fs,
    process::{Command, Stdio},
};

// Enum for returning chrome version
// Could be &str or u32
pub enum Version<'v> {
    Nb(u32),
    Str(&'v str),
}

// Implement Display (needed to print version)
impl Display for Version<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Version::Nb(a) => write!(formatter, "{}", a),
            Version::Str(a) => write!(formatter, "{}", a),
        }
    }
}

#[derive(Debug)]
pub struct Utils {}

impl Utils {
    pub fn regex_helper(&self, text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(http://www\.|https://www\.|http://|https://)?[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,5}(:[0-9]{1,5})?(/.*)?$").unwrap();
        }
        RE.is_match(text)
    }
    pub fn is_in_path(&self, program: &str) -> bool {
        if let Ok(path) = env::var("PATH") {
            for p in path.split(":") {
                let p_str = format!("{}/{}", p, program);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_chrome_version(&self) -> Version<'static> {
        let command = Command::new("chromium")
            .args(&["--version"])
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        let version = String::from_utf8(command.stdout).unwrap();

        match version {
            ref v if v.contains("108") => return Version::Nb(108),
            ref v if v.contains("110") => return Version::Nb(110),
            ref v if v.contains("111") => return Version::Nb(111),
            _ => return Version::Str("Undefined chrome version"),
        };
    }

    pub fn check_os(&self) -> &str {
        return env::consts::OS;
    }
}
