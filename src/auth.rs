//!
//! Basic and Token Authentication Credentials
//!


/// Credentials used to authenticate at the InfluxDB server
#[derive(Debug)]
pub enum Credentials
{
    /// HTTP Basic authentication pattern. This pattern authenticates at the server and receives
    /// back the token to use for subsequent queries against the API.
    Basic {user: String, passwd: String, cookie: Option<String>},

    /// Provide token generated directly in the InfluxDB GUI or CLI.
    Token {token: String},
}


impl Credentials
{
    /// User and password for HTTP basic auth at the server API
    pub fn from_basic(user: &str, passwd: &str) -> Self
    {
        Self::Basic {
            user:   user.to_owned(),
            passwd: passwd.to_owned(),
            cookie: None,
        }
    }

    /// Token to utilize for requests at the server API
    pub fn from_token(token: &str) -> Self
    {
        Self::Token {
            token: token.to_owned(),
        }
    }
}
