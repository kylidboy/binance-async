use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Client as rqwstClient;
use serde::Deserialize;
use sha2::Sha256;

use crate::endpoints::{Endpoint, EndpointRequest, Method, Response};
use crate::errors::*;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: url::Url,
    inner_client: rqwstClient,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: &str) -> Self {
        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host: url::Url::parse(host).expect("malformed host string"),
            inner_client: rqwstClient::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn get_keys(&self) -> (&str, &str) {
        (&self.api_key, &self.secret_key)
    }

    pub async fn access<R: EndpointRequest>(
        &self,
        endpoint: &(dyn Endpoint + Send + Sync),
        data: Option<R>,
    ) -> Result<R::Response> {
        let (method, security, endpoint_path) = endpoint.action_params();
        let ds: String;
        let query_str = if data.is_some() {
            ds = data.unwrap().to_string();
            if ds.is_empty() {
                None
            } else {
                Some(ds.as_str())
            }
        } else {
            None
        };
        match method {
            Method::GET => match security {
                crate::endpoints::SecurityType::None => {
                    self.get::<R::Response>(&endpoint_path, query_str).await
                }
                crate::endpoints::SecurityType::Trade
                | crate::endpoints::SecurityType::Margin
                | crate::endpoints::SecurityType::UserData => {
                    self.get_signed::<R::Response>(&endpoint_path, query_str)
                        .await
                }
                crate::endpoints::SecurityType::UserStream
                | crate::endpoints::SecurityType::MarketData => {
                    self.get_key_only::<R::Response>(&endpoint_path, query_str)
                        .await
                }
            },
            Method::POST => match security {
                crate::endpoints::SecurityType::None => {
                    self.post::<R::Response>(&endpoint_path, query_str).await
                }
                crate::endpoints::SecurityType::UserData
                | crate::endpoints::SecurityType::Trade
                | crate::endpoints::SecurityType::Margin => {
                    self.post_signed::<R::Response>(&endpoint_path, query_str)
                        .await
                }
                crate::endpoints::SecurityType::UserStream
                | crate::endpoints::SecurityType::MarketData => {
                    self.post_key_only(&endpoint_path, query_str).await
                }
            },
            Method::PUT => match security {
                crate::endpoints::SecurityType::None => todo!(),
                crate::endpoints::SecurityType::UserData
                | crate::endpoints::SecurityType::Trade
                | crate::endpoints::SecurityType::Margin => todo!(),
                crate::endpoints::SecurityType::UserStream
                | crate::endpoints::SecurityType::MarketData => todo!(),
            },
            Method::DELETE => match security {
                crate::endpoints::SecurityType::None => todo!(),
                crate::endpoints::SecurityType::UserData
                | crate::endpoints::SecurityType::Trade
                | crate::endpoints::SecurityType::Margin => todo!(),
                crate::endpoints::SecurityType::UserStream
                | crate::endpoints::SecurityType::MarketData => todo!(),
            },
            _ => unimplemented!(),
        }
    }

    pub async fn get<T>(&self, endpoint: &str, query_string: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url = self.host.join(endpoint)?;
        url.set_query(query_string);
        let response = self
            .inner_client
            .clone()
            .get(url)
            .headers(self.build_headers(true)?)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }
    pub async fn get_key_only<T>(&self, endpoint: &str, query_string: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url = self.host.join(endpoint)?;
        url.set_query(query_string);
        let sig_param = self.sign_request(url.query());
        url.set_query(Some(&sig_param));
        let response = self
            .inner_client
            .clone()
            .get(url)
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }

    pub async fn get_signed<T>(&self, endpoint: &str, request: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url = self.host.join(endpoint)?;
        let signed_request = self.sign_request(request);
        url.set_query(Some(&signed_request));
        let response = self
            .inner_client
            .clone()
            .get(url)
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }

    pub async fn post<T>(&self, endpoint: &str, data: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url = self.host.join(endpoint)?;
        url.set_query(data);
        let response = self
            .inner_client
            .post(url)
            .headers(self.build_headers(true)?)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }

    pub async fn post_key_only<T>(&self, endpoint: &str, data: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url = self.host.join(endpoint)?;
        url.set_query(data);
        let response = self
            .inner_client
            .post(url)
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }

    pub async fn post_signed<T>(&self, endpoint: &str, request: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url = self.host.join(endpoint)?;
        url.set_query(request);
        let sig_param = self.sign_request(url.query());
        url.set_query(Some(&sig_param));
        let response = self
            .inner_client
            .clone()
            .post(url)
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }

    pub async fn delete<T>(&self, endpoint: &str, listen_key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.host.join(endpoint).unwrap();
        let data: String = format!("listenKey={}", listen_key);

        let response = self
            .inner_client
            .delete(url)
            .headers(self.build_headers(true)?)
            .body(data)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }
    pub async fn delete_signed<T>(&self, endpoint: &str, request: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url = self.host.join(endpoint)?;
        url.set_query(request);
        let sig_param = self.sign_request(url.query());
        url.set_query(Some(&sig_param));

        let response = self
            .inner_client
            .clone()
            .delete(url)
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }

    pub async fn put<T>(&self, endpoint: &str, listen_key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.host.join(endpoint).unwrap();
        let data: String = format!("listenKey={}", listen_key);
        let response = self
            .inner_client
            .put(url)
            .headers(self.build_headers(true)?)
            .body(data)
            .send()
            .await?;

        Self::handle_api_return(response).await
    }

    // Request must be signed
    fn sign_request(&self, request: Option<&str>) -> String {
        request.map_or_else(
            || {
                let signed_key =
                    Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
                let signature = hex_encode(signed_key.finalize().into_bytes());
                format!("signature={}", signature)
            },
            |req| {
                let mut signed_key =
                    Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();

                signed_key.update(req.as_bytes());
                let signature = hex_encode(signed_key.finalize().into_bytes());
                format!("{req}&signature={signature}")
            },
        )
    }

    fn build_headers(&self, no_key: bool) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
        custom_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        if !no_key {
            custom_headers.insert(
                HeaderName::from_static("x-mbx-apikey"),
                HeaderValue::from_str(self.api_key.as_str())
                    .map_err(|_| BinanceApiError::InvalidHeaderValue)?,
            );
        }

        Ok(custom_headers)
    }

    async fn handle_api_return<T>(response: reqwest::Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let body = response.text().await?;
        let result = match serde_json::from_str::<Response<T>>(&body) {
            Ok(res) => res,
            Err(e) => {
                println!("FailedToDecodeResponseBody: {}", body);
                return Err(e.into());
            }
        };

        match result {
            Response::Error { code, msg } => Err(BinanceApiError::ApiReturnError(code, msg)),
            Response::Data(t) => Ok(t),
        }
    }

    // async fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
    // match response.status() {
    //     StatusCode::OK => Ok(response.json::<T>().await?),
    //     StatusCode::INTERNAL_SERVER_ERROR => {
    //         bail!("Internal Server Error");
    //     }
    //     StatusCode::SERVICE_UNAVAILABLE => {
    //         bail!("Service Unavailable");
    //     }
    //     StatusCode::UNAUTHORIZED => {
    //         bail!("Unauthorized");
    //     }
    //     StatusCode::BAD_REQUEST => {
    //         let error: BinanceContentError = response.json()?;
    //
    //         Err(ErrorKind::BinanceError(error).into())
    //     }
    //     s => {
    //         bail!(format!("Received response: {:?}", s));
    //     }
    // }
    // }
}
