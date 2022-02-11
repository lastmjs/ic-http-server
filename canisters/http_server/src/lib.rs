// I, Jordan Last, have modified much of the original Apache-licensed code

mod rc_bytes;

use crate::rc_bytes::RcBytes;

use ic_cdk_macros::query;
use serde_bytes::ByteBuf;
use ic_cdk::export::candid::{CandidType, Deserialize, Func, Nat, export_service, candid_method};

type HeaderField = (String, String);

// TODO is this type contained in some library so I don't have to manually define it?
#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: RcBytes,
    streaming_strategy: Option<StreamingStrategy>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
enum StreamingStrategy {
    Callback {
        callback: Func,
        token: StreamingCallbackToken,
    },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct StreamingCallbackToken {
    key: String,
    content_encoding: String,
    index: Nat,
    // We don't care about the sha, we just want to be backward compatible.
    sha256: Option<ByteBuf>,
}

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    // ic_cdk::println!("req.headers {:#?}", req.headers);

    // let range_request_header = req.headers.iter().find(|header| {
    //     header.0 == "range"
    // }).unwrap();

    let request_header_names: Vec<String> = req.headers.into_iter().map(|header| header.0).collect();

    HttpResponse {
        status_code: 206,
        headers: vec![
            ("Content-Range".to_string(), "bytes 0-2/3".to_string()),
            ("Request-Header-Names".to_string(), request_header_names.join(","))
            // ("Range-Request-Header".to_string(), range_request_header.1.clone().to_string())
        ],
        body: RcBytes::from(ByteBuf::from(vec![0, 1, 2])),
        streaming_strategy: None
    }
}

export_service!();

#[query]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}