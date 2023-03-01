use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     UsersByIdV1GetResponse,
     UsersV1GetResponse
};

// ↓　ルータ？
mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v1/users$",
            r"^/v1/users/(?P<user_id>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_V1_USERS: usize = 0;
    pub(crate) static ID_V1_USERS_USER_ID: usize = 1;
    lazy_static! {
        pub static ref REGEX_V1_USERS_USER_ID: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/users/(?P<user_id>[^/?#]*)$")
                .expect("Unable to create regex for V1_USERS_USER_ID");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

// エラーレスポンス(405)
fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    // この中の該当の関数にハンドラなどを追加していく　
    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // UsersByIdV1Get - GET /v1/users/{user_id}
            hyper::Method::GET if path.matched(paths::ID_V1_USERS_USER_ID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_USERS_USER_ID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_USERS_USER_ID in set but failed match against \"{}\"", path, paths::REGEX_V1_USERS_USER_ID.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["user_id"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<String>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter user_id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["user_id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.users_by_id_v1_get(
                                            param_user_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UsersByIdV1GetResponse::Status200
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for USERS_BY_ID_V1_GET_STATUS200"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UsersByIdV1GetResponse::Status404
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("applicatoin/json")
                                                            .expect("Unable to create Content-Type header for USERS_BY_ID_V1_GET_STATUS404"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UsersByIdV1GetResponse::Status500
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("applicatoin/json")
                                                            .expect("Unable to create Content-Type header for USERS_BY_ID_V1_GET_STATUS500"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UsersV1Get - GET /v1/users
            hyper::Method::GET if path.matched(paths::ID_V1_USERS) => {
                                println!("called /v1/users !!!");
                                
                                // ↓ハンドラ？
                                let result = api_impl.users_v1_get(
                                        &context
                                    ).await;
                                
                                // 空のレスポンス生成
                                let mut response = Response::new(Body::empty());
                                
                                // 生成したレスポンスに返したいものを入れていく
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            // 成功時のレスポンス(エラーレスポンス含む)
                                            Ok(rsp) => match rsp {
                                                // ステータスコードが200
                                                UsersV1GetResponse::Status200
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for USERS_V1_GET_STATUS200"));
                                                    // let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    let body = serde_json::to_string("{'msg': 'Success!!!!'}").expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                // ステータスコードが500
                                                UsersV1GetResponse::Status500
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("applicatoin/json")
                                                            .expect("Unable to create Content-Type header for USERS_V1_GET_STATUS500"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            // 失敗時のレスポンス
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                let body = serde_json::to_string("{'msg': 'Success!!!!'}").expect("impossible to fail to serialize");
                                                *response.body_mut() = Body::from(body);
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_V1_USERS) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_USERS_USER_ID) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // UsersByIdV1Get - GET /v1/users/{user_id}
            hyper::Method::GET if path.matched(paths::ID_V1_USERS_USER_ID) => Some("UsersByIdV1Get"),
            // UsersV1Get - GET /v1/users
            hyper::Method::GET if path.matched(paths::ID_V1_USERS) => Some("UsersV1Get"),
            _ => None,
        }
    }
}
