//!
//! Basic and Token Authentication Credentials
//!


#[derive(Debug)]
pub enum Credentials
{
    Basic {user: String, passwd: String, cookie: Option<String>},
    Token {token: String},
}


impl Credentials
{
    pub fn from_basic(user: &str, passwd: &str) -> Self
    {
        Self::Basic {
            user:   user.to_owned(),
            passwd: passwd.to_owned(),
            cookie: None,
        }
    }

    pub fn from_token(token: &str) -> Self
    {
        Self::Token {
            token: token.to_owned(),
        }
    }
}
