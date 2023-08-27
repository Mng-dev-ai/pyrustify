use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;

const SIGNUP_PAGE: &str = "https://login.yahoo.com/account/create?specId=yidReg&lang=en-US&src=&done=https%3A%2F%2Fwww.yahoo.com&display=login";
const SIGNUP_API: &str = "https://login.yahoo.com/account/module/create?validateField=yid";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36";

lazy_static! {
    static ref ACRUMB_REGEX: Regex = Regex::new(r"s=(?P<capture>[^;]*)&d").unwrap();
    static ref SESSION_INDEX_REGEX: Regex =
        Regex::new(r#"<input type="hidden" value="(?P<capture>.*)" name="sessionIndex">"#).unwrap();
}

// Inspired by https://github.com/reacherhq/check-if-email-exists/blob/master/core/src/smtp/yahoo.rs
pub fn check_yahoo(email: &str) -> Option<bool> {
    let client = Client::builder().user_agent(USER_AGENT).build().ok()?;

    let resp = client.get(SIGNUP_PAGE).send().ok()?;
    let cookies = resp.headers().get("Set-Cookie")?.to_str().ok()?.to_owned();
    let body = resp.text().ok()?;

    let get_capture = |regex: &Regex, text: &str| {
        regex
            .captures(text)?
            .name("capture")
            .map(|c| c.as_str().to_string())
    };

    let form_data = [
        ("acrumb", get_capture(&ACRUMB_REGEX, &cookies)?),
        ("sessionIndex", get_capture(&SESSION_INDEX_REGEX, &body)?),
        ("specId", "yidReg".to_string()),
        ("userId", email.split('@').next()?.into()),
    ];

    let resp = client
        .post(SIGNUP_API)
        .header("Origin", "https://login.yahoo.com")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Cookie", &cookies)
        .form(&form_data)
        .send()
        .ok()?;

    Some(!resp.text().ok()?.contains("IDENTIFIER_NOT_AVAILABLE"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert_eq!(check_yahoo("admin@yahoo.com").unwrap(), true);
    }

    #[test]
    fn test_invalid_email() {
        assert_eq!(check_yahoo("nonexistingemail@yahoo.com").unwrap(), false);
    }
}
