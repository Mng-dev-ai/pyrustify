use crate::mx::*;
use crate::settings::Settings;
use crate::utils::is_valid;

use async_native_tls::TlsConnector;
use async_smtp::{
    smtp::{commands::*, extension::ClientId, ClientSecurity, ServerAddress},
    ClientTlsParameters, EmailAddress, SmtpClient,
};
use pyo3::{types::PyDict, PyObject, Python, ToPyObject};
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub(crate) struct Smtp {
    is_deliverable: Option<bool>,
}
impl Smtp {
    pub(crate) fn new(email: &str) -> Self {
        Self {
            is_deliverable: is_deliverable(email),
        }
    }
}

impl ToPyObject for Smtp {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("is_deliverable", self.is_deliverable.to_object(py))
            .unwrap();
        dict.to_object(py)
    }
}

fn get_smtp_server(email: &str) -> Option<String> {
    let response = match mx_lookup(email) {
        Ok(response) => response,
        Err(_) => return None,
    };
    // the preference method represents the priority of the MX record, the lower the number the higher the priority
    let mx_record = response.iter().min_by_key(|mx| mx.preference()).unwrap();
    Some(mx_record.exchange().to_ascii())
}

pub(crate) fn is_deliverable(email: &str) -> Option<bool> {
    if !is_valid(email) {
        return None;
    }
    let smtp_server = match get_smtp_server(email) {
        Some(smtp_server) => smtp_server,
        None => return None,
    };
    let runtime = Runtime::new().unwrap();
    let settings = Settings::new();
    let security = ClientSecurity::Opportunistic(ClientTlsParameters::new(
        smtp_server.clone(),
        TlsConnector::new()
            .use_sni(true)
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true),
    ));
    let smtp_client = SmtpClient::with_security(
        ServerAddress::new(smtp_server, settings.smtp_port.try_into().unwrap()),
        security,
    )
    .hello_name(ClientId::Domain(settings.hello_name))
    .timeout(Some(Duration::from_secs(
        settings.smtp_timeout.try_into().unwrap(),
    )));
    let mut smtp_transport = smtp_client.into_transport();
    // try to connect to the server
    let _response = runtime.block_on(smtp_transport.connect());
    // first try to send a MAIL command
    let from_email = EmailAddress::new(settings.from_email).unwrap();
    let _response =
        runtime.block_on(smtp_transport.command(MailCommand::new(Some(from_email), vec![])));
    // then try to send a RCPT command
    let to_email = EmailAddress::new(email.to_string()).unwrap();
    let response = runtime.block_on(smtp_transport.command(RcptCommand::new(to_email, vec![])));
    // finally send a QUIT command
    let _response = runtime.block_on(smtp_transport.command(QuitCommand));
    // if the RCPT command was successful, the email is deliverable
    Some(response.is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_smtp_server() {
        assert_eq!(
            get_smtp_server("test@gmail.com"),
            Some("gmail-smtp-in.l.google.com.".to_string())
        );
    }
    #[test]
    fn test_get_smtp_server_for_invalid_email() {
        assert_eq!(get_smtp_server("test"), None);
    }
    #[test]
    fn test_check_smtp_for_invalid_email() {
        assert_eq!(is_deliverable("test"), None);
    }
    #[test]
    fn test_check_smtp_for_existing_email() {
        assert_eq!(is_deliverable("nagymichel13@gmail.com"), Some(true));
    }
    #[test]
    fn test_check_smtp_for_non_existing_email() {
        assert_eq!(is_deliverable("nonexistingemail@example.org"), Some(false));
    }
}
