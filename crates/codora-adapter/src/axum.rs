//! Document codora support for axum framework
//!
use axum::{
    extract::FromRequestParts,
    http::{Extensions, request::Parts},
};
use codora_security::AuthnContext;
use http::StatusCode;

#[derive(Clone)]
pub struct Context {
    // This is cheap to clone in axum codebase so we could have without arc
    extension: Extensions,
}

impl Context {
    pub fn new() -> Self {
        let extension = Extensions::default();
        Self { extension }
    }
}

impl<S> FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Context>()
            .cloned()
            // ofc change this to handle error perfectly but this is something we wanna do
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl AuthnContext for Context {
    fn register_extension<T>(&mut self, ext: T) -> &mut Self
    where
        T: Send + Sync + 'static + Clone,
    {
        self.extension.insert(ext);
        self
    }

    fn get_extension<T>(&self) -> Option<&T>
    where
        T: Send + Sync + 'static + Clone,
    {
        self.extension.get::<T>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{Extension, Router, body::Body, routing::get};
    use codora_security::{Cookie, State as CookieState};
    use http::Request;
    use tower::ServiceExt as _;
    use tracing_subscriber::EnvFilter;

    #[tokio::test]
    async fn test_context() -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("trace"))
            .try_init();

        async fn handler(ctx: Context) -> ((), &'static str) {
            trace!("Got called !!!");
            // properties
            // handler + payload

            // This should be simplify further as cookie state should be easily derived either from extension or
            let cookie_state = CookieState {};

            //  we use usnit here as payload but you get the idea this is how we wanna authenticate
            if let Ok(res) = ctx
                .sign_in::<Cookie>(cookie_state, ())
                .await
            {
                return (res, "Hello welcome you've been authenticated!");
            }
            // we wanna do something like this but would be nice if we could have

            // pass properties and payload only no need of the generic handler
            // let response = ctx.sign_in_with_cookie().await?;
            ((), "Failed to authenticate user")
        }

        let auth_context = Context::new().configure::<Cookie>(|option| {
            trace!("Yes we bout to configure options here");
            option
        });

        let app: Router<()> = Router::new()
            .route("/", get(handler))
            // .layer(ContextLayer::new(auth_context));
            // let just use extension for now
            .layer(Extension(auth_context));

        let request = Request::builder()
            .uri("/")
            .method("GET")
            .body(Body::empty())?;

        app.oneshot(request).await?;
        Ok(())
    }
}
