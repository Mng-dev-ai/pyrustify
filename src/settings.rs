fn get_string_from_env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}

fn get_int_from_env(key: &str, default: i32) -> i32 {
    match std::env::var(key) {
        Ok(val) => val.parse().unwrap_or(default),
        Err(_) => default,
    }
}

#[derive(Debug)]
pub struct Settings {
    pub from_email: String,
    pub hello_name: String,
    pub smtp_port: i32,
    pub smtp_timeout: i32,
    pub check_smtp: bool,
    pub check_mx: bool,
    pub check_misc: bool,
}

impl Settings {
    pub fn new() -> Self {
        let from_email = get_string_from_env("FROM_EMAIL", "user@example.org");
        let hello_name = get_string_from_env("HELLO_NAME", "localhost");
        let smtp_port = get_int_from_env("SMTP_PORT", 25);
        let smtp_timeout = get_int_from_env("SMTP_TIMEOUT", 10);
        let check_smtp = match std::env::var("CHECK_SMTP") {
            Ok(val) => val == "true",
            Err(_) => false,
        };
        let check_mx = match std::env::var("CHECK_MX") {
            Ok(val) => val == "true",
            Err(_) => false,
        };
        let check_misc = match std::env::var("CHECK_MISC") {
            Ok(val) => val == "true",
            Err(_) => false,
        };
        Settings {
            from_email,
            hello_name,
            smtp_port,
            smtp_timeout,
            check_smtp,
            check_mx,
            check_misc,
        }
    }
}
