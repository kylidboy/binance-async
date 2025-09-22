pub mod market_streams;
pub mod stream_events;

use eyre::Result;
use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::task::Poll;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use url::Url;

use crate::{api_config::ApiConfig, errors::BinanceApiError};

use market_streams::MarketStreams;
use stream_events::BinanceStreamEvent;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StreamEvent {
    Ping(i64),
    CombinedStreamPayload(Box<CombinedStreamPayload>),
    RawStreamPayload(BinanceStreamEvent),
}

impl TryFrom<&str> for StreamEvent {
    type Error = BinanceApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(value)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedStreamPayload {
    pub stream: String,
    pub data: BinanceStreamEvent,
}

impl TryFrom<&str> for CombinedStreamPayload {
    type Error = BinanceApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(value)?)
    }
}

pub struct WssConnection(pub WebSocketStream<MaybeTlsStream<TcpStream>>);

impl Deref for WssConnection {
    type Target = WebSocketStream<MaybeTlsStream<TcpStream>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for WssConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WssConnection {
    async fn connect_wss(wss: &str) -> Result<Self, BinanceApiError> {
        let url = Url::parse(wss).expect("wrong wss url");
        match connect_async(url.as_str()).await {
            Ok(answer) => Ok(Self(answer.0)),
            Err(e) => Err(BinanceApiError::WsHandShake(e)),
        }
    }

    #[allow(dead_code)]
    async fn disconnect(&mut self) -> Result<(), BinanceApiError> {
        Ok(self.0.close(None).await?)
    }
}

#[allow(dead_code)]
pub struct RawStream {
    name: MarketStreams,
    connection: WssConnection,
}

impl RawStream {
    pub async fn new(
        config: &ApiConfig,
        market_stream: MarketStreams,
    ) -> Result<Self, BinanceApiError> {
        let stream_name: String = (&market_stream).into();
        Ok(Self {
            name: market_stream,
            connection: WssConnection::connect_wss(&format!(
                "{}/ws/{}",
                config.ws_endpoint, stream_name
            ))
            .await?,
        })
    }
}

impl Stream for RawStream {
    type Item = BinanceStreamEvent;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let s = &mut self.get_mut().connection; // .as_mut().unwrap();

        match s.poll_next_unpin(cx) {
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(x) => {
                let x = x.unwrap();
                let x = x.unwrap();
                match x.to_text() {
                    Ok(msg) => {
                        let data = match msg.try_into() {
                            Ok(d) => d,
                            Err(_) => {
                                return Poll::Ready(Some(BinanceStreamEvent::Reconnect));
                            }
                        };
                        match data {
                            StreamEvent::Ping(_ts) => Poll::Ready(None),
                            StreamEvent::CombinedStreamPayload(_) => {
                                unreachable!()
                            }
                            StreamEvent::RawStreamPayload(event) => Poll::Ready(Some(event)),
                        }
                    }
                    Err(e) => {
                        panic!("RawStream payload: {}", e);
                    }
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct CombinedStream {
    pub name: Vec<MarketStreams>,
    connection: Option<WssConnection>,
}

impl CombinedStream {
    pub async fn new(
        ws_endpoint: String,
        streams: Vec<MarketStreams>,
    ) -> Result<Self, BinanceApiError> {
        let streams_names = streams
            .iter()
            .map(|s| s.into())
            .collect::<Vec<String>>()
            .join("/");
        Ok(Self {
            name: streams,
            connection: Some(
                WssConnection::connect_wss(&format!(
                    "{}/stream?streams={}",
                    ws_endpoint, streams_names
                ))
                .await?,
            ),
        })
    }

    pub async fn disconnect(mut self) -> Result<(), BinanceApiError> {
        let mut x = Option::take(&mut self.connection).unwrap();
        Ok(x.close(None).await?)
    }
}

impl Stream for CombinedStream {
    type Item = CombinedStreamPayload;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let stream_name = self
            .name
            .iter()
            .map(|v| v.into())
            .collect::<Vec<String>>()
            .join("/");
        let s = self.get_mut().connection.as_mut().unwrap();

        match s.poll_next_unpin(cx) {
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(x) => {
                let x = x.unwrap();
                if x.is_err() {
                    return Poll::Ready(Some(CombinedStreamPayload {
                        stream: stream_name,
                        data: BinanceStreamEvent::Reconnect,
                    }));
                }
                let x = x.unwrap();
                match x.to_text() {
                    Ok(msg) => {
                        let data = match msg.try_into() {
                            Ok(d) => d,
                            Err(_) => {
                                return Poll::Ready(Some(CombinedStreamPayload {
                                    stream: stream_name,
                                    data: BinanceStreamEvent::Reconnect,
                                }));
                            }
                        };
                        match data {
                            StreamEvent::Ping(_ts) => Poll::Ready(None),
                            StreamEvent::CombinedStreamPayload(event) => Poll::Ready(Some(*event)),
                            StreamEvent::RawStreamPayload(_) => {
                                unreachable!()
                            }
                        }
                    }
                    Err(e) => {
                        panic!("CombinedStream payload: {}", e);
                    }
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
