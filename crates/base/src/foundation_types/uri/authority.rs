use super::{
    characters::{USER_INFO_NOT_PCT_ENCODED, REG_NAME_NOT_PCT_ENCODED},
    context::Context,
    error::Error,
    codec::{decode_element, encode_element},
    port::parse_host_port,
    ipv6_address::validate_ipv6_address,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Authority {
    user_info: Option<Vec<u8>>,
    host: Vec<u8>,
    port: Option<u16>,
}

impl Authority {
    /// Borrow the host name part of the Authority.
    #[must_use = "host not used"]
    pub fn host(&self) -> &[u8] {
        &self.host
    }

    /// Borrow the port number part of the Authority.
    #[must_use = "port not used"]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Change the `user_info` part of the Authority.
    pub fn set_user_info<T>(&mut self, user_info: T)
    where
        T: Into<Option<Vec<u8>>>,
    {
        self.user_info = user_info.into();
    }

    /// Change the host name part of the Authority.
    pub fn set_host<T>(&mut self, host: T)
    where
        T: Into<Vec<u8>>,
    {
        self.host = host.into();
    }

    /// Change the port number part of the Authority.
    pub fn set_port(&mut self, port: Option<u16>) {
        self.port = port;
    }

    /// Borrow the `user_info` part of the Authority.
    #[must_use = "user_info not used"]
    pub fn user_info(&self) -> Option<&[u8]> {
        self.user_info.as_deref()
    }

    /// Interpret the given string as the Authority component of a URI,
    /// separating its various subcomponents, returning an `Authority` value
    /// containing them.
    ///
    /// # Errors
    ///
    /// There are many ways to screw up the Authority part of URI string, and
    /// this function will let you know what's up by returning a variant of the
    /// [`Error`](enum.Error.html) type.
    pub fn parse<T>(authority_string: T) -> Result<Self, Error>
    where
        T: AsRef<str>,
    {
        let (user_info, host_port_string) = Self::parse_user_info(authority_string.as_ref())?;
        let (host, port) = parse_host_port(host_port_string)?;
        Ok(Self {
            user_info,
            host,
            port,
        })
    }

    fn parse_user_info(authority: &str) -> Result<(Option<Vec<u8>>, &str), Error> {
        Ok(match authority.find('@') {
            Some(delimiter) => (
                Some(decode_element(
                    &authority[0..delimiter],
                    &USER_INFO_NOT_PCT_ENCODED,
                    Context::UserInfo,
                )?),
                &authority[delimiter + 1..],
            ),
            None => (None, authority),
        })
    }
}

impl std::fmt::Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(user_info) = &self.user_info {
            write!(
                f,
                "{}@",
                encode_element(user_info, &USER_INFO_NOT_PCT_ENCODED)
            )?;
        }
        let host_to_string = String::from_utf8(self.host.clone());
        match host_to_string {
            Ok(host_to_string) if validate_ipv6_address(&host_to_string).is_ok() => {
                write!(f, "[{}]", host_to_string.to_ascii_lowercase())?;
            }
            _ => {
                write!(
                    f,
                    "{}",
                    encode_element(&self.host, &REG_NAME_NOT_PCT_ENCODED)
                )?;
            }
        }
        if let Some(port) = self.port {
            write!(f, ":{}", port)?;
        }
        Ok(())
    }
}