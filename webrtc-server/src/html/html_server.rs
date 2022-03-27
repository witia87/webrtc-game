use std::net::SocketAddr;
use hyper::{Body, header, Method, Response, StatusCode, Error, Server};
use hyper::header::HeaderValue;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use webrtc_unreliable::SessionEndpoint;

pub fn start_hosting(session_endpoint: SessionEndpoint,
                     session_listen_addr: SocketAddr) {
    let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
        let session_endpoint = session_endpoint.clone();
        let remote_addr = addr_stream.remote_addr();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let mut session_endpoint = session_endpoint.clone();
                async move {
                    if req.uri().path() == "/"
                        || req.uri().path() == "/index.html" && req.method() == Method::GET
                    {
                        log::info!("serving example index HTML to {}", remote_addr);
                        Response::builder().body(Body::from(include_str!("echo_server.html")))
                    } else if req.uri().path() == "/new_rtc_session" && req.method() == Method::POST
                    {
                        log::info!("WebRTC session request from {}", remote_addr);
                        match session_endpoint.http_session_request(req.into_body()).await {
                            Ok(mut resp) => {
                                resp.headers_mut().insert(
                                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                    HeaderValue::from_static("*"),
                                );
                                Ok(resp.map(Body::from))
                            }
                            Err(err) => {
                                log::warn!("bad rtc session request: {:?}", err);
                                Response::builder()
                                    .status(StatusCode::BAD_REQUEST)
                                    .body(Body::from(format!("error: {:?}", err)))
                            }
                        }
                    } else {
                        Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("not found"))
                    }
                }
            }))
        }
    });

    tokio::spawn(async move {
        Server::bind(&session_listen_addr)
            .serve(make_svc)
            .await
            .expect("HTTP session server has died");
    });
}
