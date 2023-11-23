mod misc;
mod mx;
mod settings;
mod smtp;
mod utils;

use crate::{misc::*, mx::*, settings::Settings, smtp::*, utils::*};

use pyo3::{conversion::ToPyObject, prelude::*, types::PyDict, wrap_pyfunction};
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
        let email = email.to_string();
        Self {
            email: email.clone(),
            has_valid_syntax: is_valid(&email),
            mx: settings.check_mx.then(|| Mx::new(&email)),
            misc: settings.check_misc.then(|| Misc::new(&email)),
            smtp: settings.check_smtp.then(|| Smtp::new(&email)),
        }
    }
}
impl ToPyObject for Result {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);

        dict.set_item("email", self.email.clone()).unwrap();
        dict.set_item("has_valid_syntax", self.has_valid_syntax)
            .unwrap();
        if let Some(mx) = self.mx.as_ref() {
            dict.set_item("mx", mx).unwrap()
        }
        if let Some(misc) = self.misc.as_ref() {
            dict.set_item("misc", misc).unwrap()
        }
        if let Some(smtp) = self.smtp.as_ref() {
            dict.set_item("smtp", smtp).unwrap()
        }
        dict.into()
    }
}
#[pyfunction]
fn verify_email(email: &str) -> PyResult<PyObject> {
    let settings = Settings::new();
    let result = Result::new(email, &settings);
    Ok(pyo3::Python::with_gil(|py| result.to_object(py)))
}

#[pyfunction]
fn verify_emails(emails: Vec<&str>) -> PyResult<PyObject> {
    let settings = Settings::new();
    let results: Vec<Result> = emails
        .par_iter() // rayon is used to parallelize the validation
        .map(|email| Result::new(email, &settings))
        .collect();
    Ok(pyo3::Python::with_gil(|py| results.to_object(py)))
}
#[pymodule]
fn pyrustify(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(verify_email))?;
    m.add_wrapped(wrap_pyfunction!(verify_emails))?;
    Ok(())
}
