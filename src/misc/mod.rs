use crate::utils::is_valid;
use pyo3::{types::PyDict, PyObject, Python, ToPyObject};
use reqwest::blocking::Client;

const DISPOSABLE_DATA_URL: &str =
    "https://raw.githubusercontent.com/7c/fakefilter/main/txt/data.txt";
const FREE_EMAIL_URL: &str = "https://gist.githubusercontent.com/okutbay/5b4974b70673dfdcc21c517632c1f984/raw/free_email_provider_domains.txt";

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
        dict.set_item("is_disposable", self.is_disposable.to_object(py))
            .unwrap();
        dict.set_item("is_free", self.is_free.to_object(py))
            .unwrap();
        dict.set_item("is_role_account", self.is_role_account.to_object(py))
            .unwrap();
        dict.to_object(py)
    }
}

fn is_disposable(email: &str) -> Option<bool> {
    if !is_valid(email) {
        return None;
    }
    let domain = email.split('@').last().unwrap();
    let client = Client::new();
    let response = client.get(DISPOSABLE_DATA_URL).send();
    match response {
        Ok(response) => {
            if response.status().is_success() {
                let text = response.text().unwrap_or_default();
                for line in text
                    .lines()
                    .filter(|l| !l.is_empty() && !l.starts_with('#'))
                {
                    if line.trim().eq_ignore_ascii_case(domain) {
                        return Some(true);
                    }
                }
                Some(false)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn is_free(email: &str) -> Option<bool> {
    if !is_valid(email) {
        return None;
    }
    let domain = email.split('@').last().unwrap();
    match Client::new().get(FREE_EMAIL_URL).send() {
        Ok(response) => {
            if response.status().is_success() {
                let text = response.text().unwrap_or_default();
                let is_free = text
                    .lines()
                    .any(|line| line.trim().eq_ignore_ascii_case(domain));
                Some(is_free)
            } else {
                None
            }
        }
        Err(_) => None,
    }
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
