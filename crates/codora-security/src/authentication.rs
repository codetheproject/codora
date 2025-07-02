pub(super) mod cookie;
pub(super) mod handler;
use handler::{Handler, SignInHandler, SignOutHandler};

pub trait AuthnContext: Sized {
    fn register_extension<T>(&mut self, ext: T) -> &mut Self
    where
        T: Send + Sync + 'static + Clone;

    fn get_extension<T>(&self) -> Option<&T>
    where
        T: Send + Sync + 'static + Clone;

    fn configure<H>(mut self, cb: impl FnOnce(H::Option) -> H::Option) -> Self
    where
        H: Handler + Send + Sync + 'static,
        H::Option: Clone,
    {
        // This look's like something we wanna do when configuring the idea is that option is registered in the context extension
        // then handler could extract it from the context
        let payload = cb(Default::default());
        self.register_extension(payload);

        self
    }

    fn sign_in<H>(&self, state: H::State, payload: H::Payload) -> impl Future<Output = Result<(), H::Error>> + Send
    where
        H: SignInHandler + Sync + Send,
        H::Option: Clone + Sync + Send,
    {
        let handler_option = self
            .get_extension::<H::Option>()
            .cloned()
            // Handle error properly
            .unwrap();

        // we've got the handler here but i think this could be problematic and less efficient
        let handler = H::from(handler_option);
        async move {
            match handler.sign_in(state, payload).await {
                Ok(res) => {
                    trace!("Signed in got: {:?}", res);
                    return Ok(());
                }
                Err(error) => {
                    todo!("Handle error properly")
                }
            }
        }
    }
}

/** given an authentication method like username and password to create a session
 * method --> ctx --> handler --> response
 *
 * method should produce what handler needed like cookie, jwt, token then handler issue a response
 *
 * for now we assumed we don't have method but we have context
 * context should be agnositic which means each framework define it's own context
 */
#[cfg(test)]
mod test {}
