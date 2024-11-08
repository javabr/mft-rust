/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for InitiateTransfer
    lazy_static! {
        pub static ref INITIATE_TRANSFER: Mime = "application/json".parse().unwrap();
    }

}
