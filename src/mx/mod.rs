use crate::utils::is_valid;
use pyo3::{types::PyDict, PyObject, Python, ToPyObject};
use trust_dns_resolver::lookup::MxLookup;
use trust_dns_resolver::{config::*, Resolver};

#[derive(Debug)]
pub(crate) struct Mx {
    has_mx_records: bool,
    mx_records: Vec<String>,
}

impl Mx {
    pub(crate) fn new(email: &str) -> Self {
        let mx_records = get_mx_records(email);
        Self {
            has_mx_records: mx_records.is_some(),
            mx_records: mx_records.unwrap_or_default(),
        }
    }
}

impl ToPyObject for Mx {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("has_mx_records", self.has_mx_records)
            .unwrap();
        dict.set_item("mx_records", self.mx_records.clone())
            .unwrap();
        dict.into()
    }
}

pub(crate) fn mx_lookup(email: &str) -> Result<MxLookup, trust_dns_resolver::error::ResolveError> {
    let domain = email.split('@').last().unwrap();
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    resolver.mx_lookup(domain)
}

pub(crate) fn get_mx_records(email: &str) -> Option<Vec<String>> {
    if !is_valid(email) {
        return None;
    }
    let response = mx_lookup(email);
    match response {
        Ok(response) => {
            let mx_records = response
                .iter()
                .map(|mx| mx.exchange().to_string())
                .collect();
            Some(mx_records)
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mx_records_for_invalid_email() {
        let mx_records = get_mx_records("example");
        assert_eq!(mx_records, None);
    }
    #[test]
    fn test_get_mx_records_for_valid_domain() {
        let mx_records = get_mx_records("example@gmail.com");
        assert_eq!(mx_records.is_some(), true);
    }
    #[test]
    fn test_get_mx_records_for_invalid_domain() {
        let mx_records = get_mx_records("example@notvaliddomain.com");
        assert_eq!(mx_records.is_some(), false);
    }
}
