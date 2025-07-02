//! # Cookie Authentication
//!
//! # Overview
//! - Cookie authentication is a method of authentication that involves using cookies to store user credentials.

use crate::authentication::handler::{Handler, SignInHandler, SignOutHandler};
mod cookies;

#[derive(Debug)]
pub enum Error {}

#[derive(Debug, Clone)]
pub struct CookieOption {}

impl Default for CookieOption {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct State {}

#[derive(Debug, Clone)]
pub struct Cookie {
    cookie_option: CookieOption,
}

impl From<CookieOption> for Cookie {
    fn from(value: CookieOption) -> Self {
        Cookie { cookie_option: value }
    }
}

impl Handler for Cookie {
    type Error = Error;
    type State = State;
    type Option = CookieOption;

    const NAME: &'static str = "Cookie";

    async fn authenticate(&self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn forbid(&self, state: Self::State) -> Result<(), Self::Error> {
        todo!()
    }

    async fn challenge(&self, state: Self::State) -> Result<(), Self::Error> {
        todo!()
    }
}

impl SignOutHandler for Cookie {
    async fn sign_out(&self, state: Self::State) -> Result<(), Self::Error> {
        todo!()
    }
}

impl SignInHandler for Cookie {
    // change this to something cookie handler needed to sign in one of it something that could turn to vec of cookies
    type Payload = ();

    async fn sign_in(&self, state: Self::State, payload: Self::Payload) -> Result<(), Self::Error> {
        trace!("Got {:?} - {:?}", state, payload);
        trace!("Done authenticating....");

        Ok(())
    }
}
