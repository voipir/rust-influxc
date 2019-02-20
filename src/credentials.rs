//!
//! Credentials used to Connect Database
//!


#[derive(Debug)]
pub struct Credentials
{
    pub user:   String,
    pub passwd: String,
}


impl Credentials
{
    pub fn new(user: String, passwd: String ) -> Self
    {
        Self {user, passwd}
    }
}
