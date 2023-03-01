#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::blacklisted_name)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UsersByIdV1GetResponse {
    /// 成功
    Status200
    (models::UsersV1Get200ResponseInner)
    ,
    /// 失敗
    Status404
    (String)
    ,
    /// 失敗
    Status500
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UsersV1GetResponse {
    /// 成功
    Status200
    (Vec<models::UsersV1Get200ResponseInner>)
    ,
    /// 失敗
    Status500
    (String)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// ユーザー詳細取得API
    async fn users_by_id_v1_get(
        &self,
        user_id: String,
        context: &C) -> Result<UsersByIdV1GetResponse, ApiError>;

    /// ユーザー一覧取得API
    async fn users_v1_get(
        &self,
        context: &C) -> Result<UsersV1GetResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// ユーザー詳細取得API
    async fn users_by_id_v1_get(
        &self,
        user_id: String,
        ) -> Result<UsersByIdV1GetResponse, ApiError>;

    /// ユーザー一覧取得API
    async fn users_v1_get(
        &self,
        ) -> Result<UsersV1GetResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// ユーザー詳細取得API
    async fn users_by_id_v1_get(
        &self,
        user_id: String,
        ) -> Result<UsersByIdV1GetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().users_by_id_v1_get(user_id, &context).await
    }

    /// ユーザー一覧取得API
    async fn users_v1_get(
        &self,
        ) -> Result<UsersV1GetResponse, ApiError>
    {
        println!("called lib.rs");
        let context = self.context().clone();
        // self.api().users_v1_get(&context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
