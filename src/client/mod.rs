#![allow(unused_extern_crates)]
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate chrono;
extern crate url;





use hyper;
use hyper::header::{Headers, ContentType};
use hyper::Uri;
use self::url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET};
use futures;
use futures::{Future, Stream};
use futures::{future, stream};
use self::tokio_core::reactor::Handle;
use std::borrow::Cow;
use std::io::{Read, Error, ErrorKind};
use std::error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::str;
use std::str::FromStr;

use mimetypes;

use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;

use swagger::{ApiError, XSpanId, XSpanIdString, Has, AuthData};

use {Api,
     InitiateTransferResponse
     };
use models;

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(input: &str, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = Uri::from_str(input)?;

    let scheme = uri.scheme().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or_else(|| ClientInitError::MissingHost)?;
    let port = uri.port().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}", scheme, host, port))
}

/// A client that implements the API by making HTTP calls out to a server.
#[derive(Clone)]
pub struct Client {
    hyper_client: Arc<Box<hyper::client::Service<Request=hyper::Request<hyper::Body>, Response=hyper::Response, Error=hyper::Error, Future=hyper::client::FutureResponse>>>,
    base_path: String,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl Client {

    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    pub fn try_new_http(handle: Handle, base_path: &str) -> Result<Client, ClientInitError> {
        let http_connector = swagger::http_connector();
        Self::try_new_with_connector::<hyper::client::HttpConnector>(
            handle,
            base_path,
            Some("http"),
            http_connector,
        )
    }

    /// Create a client with a TLS connection to the server.
    ///
    /// # Arguments
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    pub fn try_new_https<CA>(
        handle: Handle,
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Client, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = swagger::https_connector(ca_certificate);
        Self::try_new_with_connector::<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>(
            handle,
            base_path,
            Some("https"),
            https_connector,
        )
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    pub fn try_new_https_mutual<CA, K, C, T>(
        handle: Handle,
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: C,
    ) -> Result<Client, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        C: AsRef<Path>,
    {
        let https_connector =
            swagger::https_mutual_connector(ca_certificate, client_key, client_certificate);
        Self::try_new_with_connector::<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>(
            handle,
            base_path,
            Some("https"),
            https_connector,
        )
    }

    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with
    /// `swagger::{http_connector, https_connector, https_mutual_connector}`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector_fn` - Function which returns an implementation of `hyper::client::Connect`
    pub fn try_new_with_connector<C>(
        handle: Handle,
        base_path: &str,
        protocol: Option<&'static str>,
        connector_fn: Box<Fn(&Handle) -> C + Send + Sync>,
    ) -> Result<Client, ClientInitError>
    where
        C: hyper::client::Connect + hyper::client::Service,
    {
        let connector = connector_fn(&handle);
        let hyper_client = Box::new(hyper::Client::configure().connector(connector).build(
            &handle,
        ));

        Ok(Client {
            hyper_client: Arc::new(hyper_client),
            base_path: into_base_path(base_path, protocol)?,
        })
    }

    /// Constructor for creating a `Client` by passing in a pre-made `hyper` client.
    ///
    /// One should avoid relying on this function if possible, since it adds a dependency on the underlying transport
    /// implementation, which it would be better to abstract away. Therefore, using this function may lead to a loss of
    /// code generality, which may make it harder to move the application to a serverless environment, for example.
    ///
    /// The reason for this function's existence is to support legacy test code, which did mocking at the hyper layer.
    /// This is not a recommended way to write new tests. If other reasons are found for using this function, they
    /// should be mentioned here.
    pub fn try_new_with_hyper_client(hyper_client: Arc<Box<hyper::client::Service<Request=hyper::Request<hyper::Body>, Response=hyper::Response, Error=hyper::Error, Future=hyper::client::FutureResponse>>>,
                                     handle: Handle,
                                     base_path: &str)
                                    -> Result<Client, ClientInitError>
    {
        Ok(Client {
            hyper_client: hyper_client,
            base_path: into_base_path(base_path, None)?,
        })
    }
}

impl<C> Api<C> for Client where C: Has<XSpanIdString> {

    fn initiate_transfer(&self, param_transfer_job_command: Option<models::TransferJobCommand>, context: &C) -> Box<Future<Item=InitiateTransferResponse, Error=ApiError>> {


        let uri = format!(
            "{}/javabr/MFT/1.0.0/jobs/transfers",
            self.base_path
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);


        // Body parameter
        let body = param_transfer_job_command.map(|ref body| {

            serde_json::to_string(body).expect("impossible to fail to serialize")
        });

if let Some(body) = body {
            request.set_body(body.into_bytes());
        }

        request.headers_mut().set(ContentType(mimetypes::requests::INITIATE_TRANSFER.clone()));
        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));




        Box::new(self.hyper_client.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                201 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            InitiateTransferResponse::TransferJobInitiated
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            InitiateTransferResponse::InvalidTransferCommand
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                409 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            InitiateTransferResponse::AnExistingTransferIsAlreadyRunning
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

}

#[derive(Debug)]
pub enum ClientInitError {
    InvalidScheme,
    InvalidUri(hyper::error::UriError),
    MissingHost,
    SslError(openssl::error::ErrorStack)
}

impl From<hyper::error::UriError> for ClientInitError {
    fn from(err: hyper::error::UriError) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl From<openssl::error::ErrorStack> for ClientInitError {
    fn from(err: openssl::error::ErrorStack) -> ClientInitError {
        ClientInitError::SslError(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &fmt::Debug).fmt(f)
    }
}

impl error::Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}
