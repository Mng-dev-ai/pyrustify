use email_address::*;

pub(crate) fn is_valid(email: &str) -> bool {
    EmailAddress::is_valid(email)
}
