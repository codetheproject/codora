use super::Error;
// Replace this with real implementation of cookie
#[derive(Debug)]
pub struct Cookie;

#[derive(Debug)]
pub struct Cookies {
    // other cookie state
    cookies: Vec<Cookie>,
}

impl<T> TryFrom<(T, Vec<Cookie>)> for Cookies {
    type Error = Error;

    fn try_from(value: (T, Vec<Cookie>)) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Vec<Cookies>> for Cookies {
    type Error = Error;

    fn try_from(value: Vec<Cookies>) -> Result<Self, Self::Error> {
        todo!()
    }
}
