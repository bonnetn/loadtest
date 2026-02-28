//! Generated protobuf types. Source: `proto/loadtest.proto`.
//!
//! Types: [`Header`], [`LoadTestConfig`], [`RequestResult`], [`WorkerStats`], [`LoadTestRunReport`]. Use [`prost::Message`] for encode/decode:
//!
//! ```ignore
//! use prost::Message;
//! use crate::proto::{LoadTestConfig, RequestResult};
//! let config = LoadTestConfig { url: "https://example.com".into(), ..Default::default() };
//! let mut buf = Vec::new();
//! config.encode(&mut buf)?;
//! let decoded = LoadTestConfig::decode(&buf[..])?;
//! ```

include!(concat!(env!("OUT_DIR"), "/loadtest.rs"));
