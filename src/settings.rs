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
    pub use_socks5: bool,
    pub socks5_host: Option<String>,
    pub socks5_port: Option<i32>,
    pub socks5_username: Option<String>,
    pub socks5_password: Option<String>,
}

impl Settings {
    pub fn new() -> Self {
        let from_email = get_string_from_env("FROM_EMAIL", "user@example.org");
        let hello_name = get_string_from_env("HELLO_NAME", "localhost");
        let smtp_port = get_int_from_env("SMTP_PORT", 25);
        let smtp_timeout = get_int_from_env("SMTP_TIMEOUT", 5);
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
        let use_socks5 = match std::env::var("USE_SOCKS5") {
            Ok(val) => val == "true",
            Err(_) => false,
        };
        let socks5_host = match std::env::var("SOCKS5_HOST") {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        let socks5_port = match std::env::var("SOCKS5_PORT") {
            Ok(val) => Some(val.parse().unwrap_or(1080)),
            Err(_) => None,
        };
        let socks5_username = match std::env::var("SOCKS5_USERNAME") {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        let socks5_password = match std::env::var("SOCKS5_PASSWORD") {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        Settings {
            from_email,
            hello_name,
            smtp_port,
            smtp_timeout,
            check_smtp,
            check_mx,
            check_misc,
            use_socks5,
            socks5_host,
            socks5_port,
            socks5_username,
            socks5_password,
        }
    }
}
