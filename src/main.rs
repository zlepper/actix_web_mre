use actix_web::{HttpRequest, FromRequest};
use actix_web::web::Bytes;
use std::pin::Pin;
use actix_web::dev::Payload;
use futures::Stream;
use actix_web::error::PayloadError;
use futures::future::{ok, TryFutureExt};

fn main() {
    println!("Hello, world!");
}

trait SessionRepository {}


impl ::actix_web::FromRequest for Box<dyn SessionRepository> {
    type Error = ::actix_web::Error;
    type Future = ::futures::future::MapOk<
        ::futures::future::Ready<Result<Self, Self::Error>>,
        FnOnce(RealSessionRepository) -> Box<(dyn SessionRepository)>,
    >;
    type Config = ();

    fn from_request(
        req: &::actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        RealSessionRepository::from_request(&req, payload).map_ok(|dep| Box::new(dep))
    }
}


struct RealSessionRepository {}

impl ::actix_web::FromRequest for RealSessionRepository {
    type Error = ::actix_web::Error;
    type Future =
        ::futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ok(RealSessionRepository {})
    }
}

impl SessionRepository for RealSessionRepository {}