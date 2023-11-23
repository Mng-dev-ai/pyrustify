use crate::{mx::*, settings::Settings, utils::is_valid};
pub mod providers;

use async_native_tls::TlsConnector;
use async_smtp::{
    smtp::{commands::*, extension::ClientId, ClientSecurity, ServerAddress, Socks5Config},
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
            is_deliverable: Self::is_deliverable(email),
        }
    }
}

impl ToPyObject for Smtp {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("is_deliverable", self.is_deliverable)
            .unwrap();
        dict.into()
    }
}
impl Smtp {
    fn get_smtp_server(email: &str) -> Option<String> {
        let response = mx_lookup(email).ok()?;
        // the preference method represents the priority of the MX record, the lower the number the higher the priority
        let mx_record = response.iter().min_by_key(|mx| mx.preference())?;
        Some(mx_record.exchange().to_ascii())
    }

    pub(crate) fn is_deliverable(email: &str) -> Option<bool> {
        if !is_valid(email) {
            return None;
        }
        let domain = email.split('@').last()?;
        if domain.eq_ignore_ascii_case("yahoo.com") {
            return providers::yahoo::check_yahoo(email);
        }
        let smtp_server = Self::get_smtp_server(email)?;
        let runtime = Runtime::new().unwrap();
        let settings = Settings::new();
        let security = ClientSecurity::Opportunistic(ClientTlsParameters::new(
            smtp_server.clone(),
            TlsConnector::new()
                .use_sni(true)
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true),
        ));
        let mut smtp_client = SmtpClient::with_security(
            ServerAddress::new(smtp_server, settings.smtp_port.try_into().unwrap()),
            security,
        )
        .hello_name(ClientId::Domain(settings.hello_name))
        .timeout(Some(Duration::from_secs(
            settings.smtp_timeout.try_into().unwrap(),
        )));
        if settings.use_socks5 {
            let socks5_config = match (settings.socks5_username, settings.socks5_password) {
                (Some(username), Some(password)) => Socks5Config::new_with_user_pass(
                    settings.socks5_host.unwrap(),
                    settings.socks5_port.unwrap() as u16,
                    username,
                    password,
                ),
                _ => Socks5Config::new(
                    settings.socks5_host.unwrap(),
                    settings.socks5_port.unwrap() as u16,
                ),
            };
            smtp_client = smtp_client.use_socks5(socks5_config);
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_smtp_server() {
        assert_eq!(
            Smtp::get_smtp_server("test@gmail.com"),
            Some("gmail-smtp-in.l.google.com.".to_string())
        );
    }
    #[test]
    fn test_get_smtp_server_for_invalid_email() {
        assert_eq!(Smtp::get_smtp_server("test"), None);
    }
    #[test]
    fn test_check_smtp_for_invalid_email() {
        assert_eq!(Smtp::is_deliverable("test"), None);
    }
    #[test]
    fn test_check_smtp_for_existing_email() {
        assert_eq!(Smtp::is_deliverable("nagymichel13@gmail.com"), Some(true));
    }
    #[test]
    fn test_check_smtp_for_non_existing_email() {
        assert_eq!(
            Smtp::is_deliverable("nonexistingemail@example.org"),
            Some(false)
        );
    }

    // #[test]
    // fn test_check_smtp_for_existing_email_with_socks5() {
    //     std::env::set_var("USE_SOCKS5", "true");
    //     assert_eq!(Smtp::is_deliverable("nagymichel13@gmail.com"), Some(true));
    // }
}
