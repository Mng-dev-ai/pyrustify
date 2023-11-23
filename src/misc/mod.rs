use crate::utils::is_valid;
use lazy_static::lazy_static;
use pyo3::{types::PyDict, PyObject, Python, ToPyObject};
use reqwest::blocking::Client;
use std::collections::HashSet;

const DISPOSABLE_DATA_URL: &str =
    "https://raw.githubusercontent.com/7c/fakefilter/main/txt/data.txt";
const FREE_EMAIL_URL: &str = "https://gist.githubusercontent.com/okutbay/5b4974b70673dfdcc21c517632c1f984/raw/free_email_provider_domains.txt";

lazy_static! {
    static ref DISPOSABLE_DOMAINS: HashSet<String> = {
        let client = Client::new();
        let response = client.get(DISPOSABLE_DATA_URL).send().unwrap();
        let text = response.text().unwrap_or_default();
        text.lines()
            .map(|line| line.trim().to_lowercase())
            .collect()
    };
    static ref FREE_EMAIL_DOMAINS: HashSet<String> = {
        let client = Client::new();
        let response = client.get(FREE_EMAIL_URL).send().unwrap();
        let text = response.text().unwrap_or_default();
        text.lines()
            .map(|line| line.trim().to_lowercase())
            .collect()
    };
}

#[derive(Debug)]
pub(crate) struct Misc {
    is_disposable: Option<bool>,
    is_free: Option<bool>,
    is_role_account: Option<bool>,
}
impl Misc {
    pub(crate) fn new(email: &str) -> Self {
        Self {
            is_disposable: is_disposable(email),
            is_free: is_free(email),
            is_role_account: is_role_account(email),
        }
    }
}
impl ToPyObject for Misc {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("is_disposable", self.is_disposable).unwrap();
        dict.set_item("is_free", self.is_free).unwrap();
        dict.set_item("is_role_account", self.is_role_account)
            .unwrap();
        dict.into()
    }
}

fn is_disposable(email: &str) -> Option<bool> {
    if !is_valid(email) {
        return None;
    }
    let domain = email.split('@').last().unwrap().to_lowercase();
    Some(DISPOSABLE_DOMAINS.contains(&domain))
}

fn is_free(email: &str) -> Option<bool> {
    if !is_valid(email) {
        return None;
    }
    let domain = email.split('@').last().unwrap().to_lowercase();
    Some(FREE_EMAIL_DOMAINS.contains(&domain))
}
fn is_role_account(email: &str) -> Option<bool> {
    if !is_valid(email) {
        return None;
    }
    let username = email.split('@').next().unwrap().to_lowercase();
    const ROLE_ACCOUNTS: &str = include_str!("role_accounts.json");
    let role_accounts: Vec<String> = serde_json::from_str(ROLE_ACCOUNTS).unwrap();
    Some(role_accounts.contains(&username))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_disposable() {
        assert_eq!(is_disposable("test@mailinator.com"), Some(true));
        assert_eq!(is_disposable("test@example.com"), Some(false));
        assert_eq!(is_disposable("invalid_email"), None);
        assert_eq!(is_disposable("test@MAILINATOR.COM"), Some(true));
    }
    #[test]
    fn test_is_free() {
        assert_eq!(is_free("example@gmail.com"), Some(true));
        assert_eq!(is_free("example@notonfreelist"), Some(false));
        assert_eq!(is_free("invalid_email"), None);
        assert_eq!(is_free("test@GMAIL.COM"), Some(true));
    }
    #[test]
    fn test_is_role_account() {
        assert_eq!(is_role_account("admin@gmail.com"), Some(true));
        assert_eq!(is_role_account("notonfreelist@gmail.com"), Some(false));
        assert_eq!(is_role_account("invalid_email"), None);
        assert_eq!(is_role_account("DEVELOPER@gmail.com"), Some(true));
    }
}
