# PyRustify

PyRustify is a Python package written in Rust that verifies the email addresses.

## Features

| Feature | Description |
|---------|-------------|
| Syntax validation | Checks if the email address has a valid syntax according to RFC 5322 |
| MX lookup | Checks if the email address has valid MX records and returns them if they exist |
| Email deliverability | Checks if the email address can receive mail by performing a SMTP handshake with the mail server |
| Misc | Checks if the email address is disposable, free or a role account using predefined lists of domains and prefixes |

## Installation

To install PyRustify, run the following command:
```
pip install pyrustify
```

## Usage

To use PyRustify, you need to import it in your Python code and call either `verify_email` or `verify_emails` function. The `verify_email` function takes a single email address as an argument and returns a dictionary with the verification results. The `verify_emails` function takes a list of email addresses as an argument and returns a list of dictionaries with the verification results for each email address.

For example:

```python
from pyrustify import verify_email, verify_emails

# Verify a single email address
response = verify_email('test@gmail.com') 
# Or a list of email addresses
response = verify_emails(['test@gmail.com','test2@gmail.com'])
print(response)
# {
    # "email": "test@gmail.com",
    # "has_valid_syntax": true,
    # "mx": {
        # "has_mx_records": true,
        # "mx_records": [
            # "alt1.gmail-smtp-in.l.google.com.",
            # "alt3.gmail-smtp-in.l.google.com.",
            # "alt4.gmail-smtp-in.l.google.com.",
            # "alt2.gmail-smtp-in.l.google.com.",
            # "gmail-smtp-in.l.google.com."
        # ]
    # },
    # "misc": {
        # "is_disposable": false,
        # "is_free": true,
        # "is_role_account": true
    # },
    # "smtp": {
        # "is_deliverable": false
    # }
# }

```

## Configuration

You can configure PyRustify by setting the following environment variables:

- `FROM_EMAIL`: The email address to use as the sender in the SMTP request. Default is `user@example.org`.
- `HELLO_NAME`: The domain name to use in the SMTP request. Default is `localhost`.
- `SMTP_PORT`: The port number to use for the SMTP request. Default is `25`.
- `SMTP_TIMEOUT`: The timeout in seconds for the SMTP request. Default is `10`.
- `CHECK_SMTP`: Whether to check the email deliverability by sending a SMTP request. Default is `false`.
- `CHECK_MX`: Whether to check the MX records of the email address. Default is `false`.
- `CHECK_MISC`: Whether to check the misc features of the email address. Default is `false`.
- `USE_SOCKS5`: Whether to use a SOCKS5 proxy for the SMTP request. Default is `false`.
- `SOCKS5_HOST`: The hostname or IP address of the SOCKS5 proxy server.
- `SOCKS5_PORT`: The port number to use for the SOCKS5 proxy server.
- `SOCKS5_USERNAME`: The username for the SOCKS5 proxy server. Optional.
- `SOCKS5_PASSWORD`: The password for the SOCKS5 proxy server. Optional.



For example, you can set these variables in your terminal before running your Python script:

```bash
export CHECK_SMTP=true
export CHECK_MX=true
export CHECK_MISC=true
```

## Credits

PyRustify uses the following sources for its miscellaneous checks:

- Disposable domains: https://github.com/7c/fakefilter
- Free email provider domains: https://gist.github.com/okutbay

## Note

To check the SMTP deliverability, you need to have port 25 open on your machine. Some ISPs may block this port by default, so you may need to contact them or use a proxy server to bypass this restriction.

## License

PyRustify is licensed under the MIT License. See [LICENSE](LICENSE) for more details.
