use reqwest::blocking::{
    Client as BClient, ClientBuilder, Request as BRequest, RequestBuilder as BRequestBuilder,
    Response as BResponse,
};
use serde::Serialize;
use std::error::Error;
use std::time::Duration;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub type Url = reqwest::Url;
pub type Response = BResponse;
pub type Request = BRequest;
pub type RequestBuilder = BRequestBuilder;
pub type Method = reqwest::Method;

pub struct Client {
    client: BClient,
    token: Option<String>,
    host_url: Option<Url>,
}

impl Client {
    fn parse_url(url: &str) -> Result<Url, Box<dyn Error>> {
        let url = url.parse::<Url>().map_err(Box::new)?;
        Ok(url)
    }

    // Check that a URL is well-formed if present.
    fn to_option_url<'h, H: Into<Option<&'h str>>>(host: H) -> Result<Option<Url>, Box<dyn Error>> {
        Ok(match host.into() {
            Some(url) => Some(Self::parse_url(url)?),
            None => None,
        })
    }

    fn new_client<D: Into<Option<Duration>>>(timeout: D) -> Result<BClient, Box<dyn Error>> {
        Ok(ClientBuilder::new()
            .user_agent(USER_AGENT)
            .timeout(timeout)
            .build()
            .map_err(Box::new)?)
    }

    pub fn new<D: Into<Option<Duration>>>(timeout: D) -> Result<Self, Box<dyn Error>> {
        Self::new_host_n_token(None, None, timeout)
    }

    pub fn new_host_n_token<'h, H, T, D>(
        host: H,
        token: T,
        timeout: D,
    ) -> Result<Self, Box<dyn Error>>
    where
        H: Into<Option<&'h str>>,
        T: Into<Option<String>>,
        D: Into<Option<Duration>>,
    {
        let host_url = Self::to_option_url(host)?;
        let client = Self::new_client(timeout)?;

        Ok(Self {
            client,
            token: token.into(),
            host_url,
        })
    }

    pub fn set_host<'h, H>(&mut self, host: H) -> Result<&mut Self, Box<dyn Error>>
    where
        H: Into<Option<&'h str>>,
    {
        self.host_url = Self::to_option_url(host)?;
        // remove the token, if any
        self.token = None;
        Ok(self)
    }

    pub fn host_url(&self) -> Option<&Url> {
        self.host_url.as_ref()
    }

    pub fn host_url_mut(&mut self) -> Option<&mut Url> {
        self.host_url.as_mut()
    }

    pub fn host_url_str(&self) -> Option<&str> {
        self.host_url().map(Url::as_str)
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_ref().map(|s| s.as_str())
    }

    pub fn set_token<S>(&mut self, token: S) -> &Self
    where
        S: Into<Option<String>>,
    {
        self.token = token.into();
        self as &Self
    }

    fn host_url_result(&self) -> Result<&Url, Box<dyn Error>> {
        self.host_url().ok_or_else(|| From::from("no url"))
    }

    pub fn request_builder<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<RequestBuilder, Box<dyn Error>>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        // Option#ok_or_else() needs inference of the (E)rror and (F)nOnce
        // parameters, and when using it in some scenarios where inference is
        // not possible, ie. without fixing the types via a return type or
        // using it with Try and a boxed trait, then it needs the explicit
        // specification of both generic types - however you can't just specify
        // E, you also must do so with F, but F is a closure, which is
        // unnameable. We can't use a trait object either (ie. dyn FnOnce),
        // since that's not the bound of F, so the solution is to either:
        //
        // 1. Use a helper that solves inference via the explicit return type
        // 2. Specify both types... using a _ for the closure.
        // 3. Use a cast:
        // let url = (self.host_url.as_ref().ok_or_else(|| {
        //         From::from("no url")
        //     }) as Result<&Url, Box<dyn Error>>)?;
        let url = self.host_url_result()?.join(path).map_err(Box::new)?;
        let mut rb = self.client.request(method, url);

        if let Some(ref token) = self.token {
            rb = rb.query(&[("access_token", token.as_str())]);
        }
        if let Some(query) = query {
            rb = rb.query(query);
        }
        if let Some(body) = body {
            rb = rb.json(body);
        }

        Ok(rb)
    }

    pub fn request<Q, B>(
        &self,
        method: Method,
        path: &str,
        query_string: Option<&Q>,
        body: Option<&B>,
    ) -> Result<Request, Box<dyn Error>>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let rb = self.request_builder(method, path, query_string, body)?;
        rb.build().map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn send_request(&self, request: Request) -> Result<Response, Box<dyn Error>> {
        self.client
            .execute(request)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn send<Q, B>(
        &self,
        method: Method,
        path: &str,
        query_string: Option<&Q>,
        body: Option<&B>,
    ) -> Result<Response, Box<dyn Error>>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        self.request_builder(method, path, query_string, body)?
            .send()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod env {
        use super::*;

        fn get_host_and_token_from_env() -> (String, String) {
            let (host, token) = (std::env::var("PORTA_URL"), std::env::var("ACCESS_TOKEN"));
            assert!(host.is_ok() && token.is_ok(), "You need to set the environment variables PORTA_URL and ACCESS_TOKEN for integration tests to run.");
            (host.unwrap(), token.unwrap())
        }

        pub fn setup_client(timeout: u8) -> Client {
            let (host, token) = get_host_and_token_from_env();
            Client::new_host_n_token(host.as_str(), token, Duration::from_secs(timeout as u64))
                .expect("failed to initalize client")
        }
    }

    use env::setup_client;

    #[test]
    fn it_generates_a_request_descriptor() {
        let c = setup_client(10);
        let req = c.request(
            Method::GET,
            "/admin/api/services/2555417783508/metrics.json",
            None::<&str>,
            None::<&str>,
        );
        assert!(req.is_ok());
    }

    #[test]
    fn it_gets_a_response() {
        let c = setup_client(10);
        let r = c.send(
            Method::GET,
            "/admin/api/services/2555417783508/metrics.json",
            None::<&u8>,
            None::<&str>,
        );
        assert!(r.is_ok());
    }
}
