use crate::{ctx::Ctx, Error, Result};
use axum::{extract::FromRequestParts, http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn middleware_authen<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("--> {:<12} - middleware_authen", "MIDDLEWARE");
    let authen_token = cookies
        .get(crate::web::AUTHEN_TOKEN)
        .map(|c| c.value().to_string());
    authen_token
        .ok_or(Error::LoadAuthenTokenFail)
        .and_then(parse_token)?;

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    println!("token: {}", token);
    let (_whole, id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::WrongAuthenTokenFormat{id: 1})?;
    let user_id = id.parse().map_err(|_| Error::WrongAuthenTokenFormat {id: 2})?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}


impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection;

        #[doc = " Perform the extraction."]
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn from_request_parts<'life0,'life1,'async_trait>(parts: &'life0 mut Parts,state: &'life1 S) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Self,Self::Rejection> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }
}