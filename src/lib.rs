mod misc;
mod mx;
mod settings;
mod smtp;
mod utils;

use crate::misc::*;
use crate::mx::*;
use crate::settings::Settings;
use crate::smtp::*;
use crate::utils::*;

use pyo3::conversion::ToPyObject;
use pyo3::types::PyDict;
use pyo3::{prelude::*, wrap_pyfunction};
use rayon::prelude::*;

#[derive(Debug)]
struct Result {
    email: String,
    has_valid_syntax: bool,
    mx: Option<Mx>,
    misc: Option<Misc>,
    smtp: Option<Smtp>,
}

impl Result {
    fn new(email: &str, settings: &Settings) -> Self {
        let has_valid_syntax = is_valid(email);
        let mx = if settings.check_mx {
            Some(Mx::new(email))
        } else {
            None
        };
        let misc = if settings.check_misc {
            Some(Misc::new(email))
        } else {
            None
        };
        let smtp = if settings.check_smtp {
            Some(Smtp::new(email))
        } else {
            None
        };
        Self {
            email: email.to_string(),
            has_valid_syntax,
            mx,
            misc,
            smtp,
        }
    }
}
impl ToPyObject for Result {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);

        dict.set_item("email", self.email.to_object(py)).unwrap();
        dict.set_item("has_valid_syntax", self.has_valid_syntax.to_object(py))
            .unwrap();
        if let Some(mx) = &self.mx {
            dict.set_item("mx", mx.to_object(py)).unwrap();
        }
        if let Some(misc) = &self.misc {
            dict.set_item("misc", misc.to_object(py)).unwrap();
        }
        if let Some(smtp) = &self.smtp {
            dict.set_item("smtp", smtp.to_object(py)).unwrap();
        }
        dict.to_object(py)
    }
}
#[pyfunction]
fn verify_email(email: &str) -> PyResult<PyObject> {
    let result = Result::new(email, &Settings::new());
    Ok(pyo3::Python::with_gil(|py| result.to_object(py)))
}

#[pyfunction]
fn verify_emails(emails: Vec<&str>) -> PyResult<PyObject> {
    let results: Vec<Result> = emails
        .par_iter() // rayon is used to parallelize the validation
        .map(|email| Result::new(email, &Settings::new()))
        .collect();
    Ok(pyo3::Python::with_gil(|py| results.to_object(py)))
}
#[pymodule]
fn pyrustify(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(verify_email))?;
    m.add_wrapped(wrap_pyfunction!(verify_emails))?;
    Ok(())
}
