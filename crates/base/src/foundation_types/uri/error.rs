use super::context::Context;

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    /// URI begins with an empty scheme, such as `://www.example.com`
    #[error("scheme expected but missing")]
    EmptyScheme,

    /// URI contains a character which is not permitted in the context where it
    /// was encountered; for example, a caret (`^`) in a query:
    /// `http://www.example.com?eat_my_^`
    #[error("illegal character in {0}")]
    IllegalCharacter(Context),

    /// URI contains an incorrect percent encoding, such as
    /// `http://www.example.com?foo=%GG`
    #[error("illegal percent encoding")]
    IllegalPercentEncoding,

    /// URI contains an invalid port number, such as
    /// `http://www.example.com:99999` or `http://www.example.com:foo`
    #[error("illegal port number")]
    IllegalPortNumber(#[source] std::num::ParseIntError),

    /// URI contains an IPv4 address with one or more bad parts, such as
    /// `http://[::ffff:1.2.3.256]/`
    #[error("octet group expected")]
    InvalidDecimalOctet,

    /// URI contains an IP address with missing parts, such as
    /// `http://[::ffff:1.2.3]/`
    #[error("too few address parts")]
    TooFewAddressParts,

    /// URI contains an IP address with too many parts, such as
    /// `http://[::ffff:1.2.3.4.8]/`
    #[error("too many address parts")]
    TooManyAddressParts,

    /// URI contains an IPv6 address with too many digits, such as
    /// `http://[20001:db8:85a3::1]/`
    #[error("too many digits in IPv6 address part")]
    TooManyDigits,

    /// URI contains an IPv6 address with more than one double-colon, such as
    /// `http://[2001:db8:85a3::8a2e::]/`
    #[error("too many double-colons in IPv6 address")]
    TooManyDoubleColons,

    /// URI contains an IPv6 address that is truncated, such as
    /// `http://[2001:db8:85a3::8a2e:0:]/`
    #[error("truncated host")]
    TruncatedHost,

    /// An AbsoluteUri was parsed without a scheme.
    #[error("missing scheme")]
    MissingScheme(
        #[source]
        #[from]
        MissingSchemeError,
    ),
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
/// An AbsoluteUri was parsed without a scheme.
pub struct MissingSchemeError {
    pub uri_string: String,
}

impl std::fmt::Display for MissingSchemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing scheme in uri: {}", self.uri_string)
    }
}