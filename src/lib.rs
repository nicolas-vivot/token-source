//! # token-source
//!
//! A lightweight library to provide common traits and structs for token sources.
//!
//! ## Token source
//!
//! A token source is anything capable of sourcing any type of token.
//! Tokens are originally meant for authentication (OAuth2, Id, Noop, etc.), but can be used for any purpose.
//!
//! ## Token source provider
//!
//! A token source provider own the token source and provides access to it.

#![warn(missing_docs)]

use std::fmt::Debug;
use std::sync::Arc;

#[cfg(feature = "async")]
use async_trait::async_trait;

/// A TokenSource abstracts how a token is obtained or generated.
///
/// This is where you would implement the logic to fetch a token from a local cache, or a remote server (for example requesting a new token from an OAuth2 server, like GKE metadata server).
#[cfg(feature = "async")]
#[async_trait]
pub trait TokenSource: Send + Sync + Debug {
    /// Returns a valid token
    async fn token(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

#[cfg(not(feature = "async"))]
pub trait TokenSource: Send + Sync + Debug {
    /// Returns a valid token
    fn token(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

/// A TokenSourceProvider provides a TokenSource implementation.
pub trait TokenSourceProvider: Send + Sync + Debug {
    /// Returns the token source implementation
    fn token_source(&self) -> Arc<dyn TokenSource>;
}

/// A Noop token source provider.
///
/// You can use this as fallback or dummy token source provider.
#[derive(Debug)]
pub struct NoopTokenSourceProvider {}

impl TokenSourceProvider for NoopTokenSourceProvider {
    fn token_source(&self) -> Arc<dyn TokenSource> {
        panic!("This is dummy token source provider. Please use any crate providing real implementation.")
    }
}
