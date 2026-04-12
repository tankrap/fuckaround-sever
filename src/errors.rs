use std::{convert::Infallible, ops::FromResidual};

use axum::response::IntoResponse;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorTypes
{
    ResourceNotFound,
    InternalServerError,
}

impl ToString for ErrorTypes
{
    fn to_string(&self) -> String
    {
        return match self {
            ErrorTypes::ResourceNotFound => "RESOURCE_NOT_FOUND",
            ErrorTypes::InternalServerError => "INTERNAL_SERVER_ERROR",
        }
        .to_string();
    }
}

pub struct ErrorResponse
{
    pub status: StatusCode,
    pub error_type: ErrorTypes,
    pub message: String,
}
pub struct OkResponse
{
    pub status: StatusCode,
    pub data: serde_json::Value,
}

pub struct ApiResult(pub anyhow::Result<OkResponse, ErrorResponse>);

impl From<anyhow::Result<OkResponse, ErrorResponse>> for ApiResult
{
    fn from(value: anyhow::Result<OkResponse, ErrorResponse>) -> Self
    {
        Self(value)
    }
}

impl FromResidual<anyhow::Result<Infallible, ErrorResponse>> for ApiResult
{
    fn from_residual(r: anyhow::Result<Infallible, ErrorResponse>) -> Self
    {
        match r {
            Err(e) => ApiResult(Err(e)),
            Ok(i) => match i {},
        }
    }
}

impl FromResidual<core::result::Result<Infallible, ssh_key::Error>>
    for ApiResult
{
    fn from_residual(
        r: core::result::Result<Infallible, ssh_key::Error>,
    ) -> Self
    {
        match r {
            Err(_e) => ApiResult(Err(ErrorResponse {
                error_type: ErrorTypes::ResourceNotFound,
                status: StatusCode::BAD_GATEWAY,
                message: "".to_string(),
            })),
            Ok(i) => match i {},
        }
    }
}

impl FromResidual<core::result::Result<Infallible, ferroid::generator::Error>>
    for ApiResult
{
    fn from_residual(
        residual: core::result::Result<Infallible, ferroid::generator::Error>,
    ) -> Self
    {
        match residual {
            Err(_err) => ApiResult(Err(ErrorResponse {
                error_type: ErrorTypes::ResourceNotFound,
                status: StatusCode::BAD_GATEWAY,
                message: "".to_string(),
            })),
            Ok(ok) => match ok {},
        }
    }
}

impl FromResidual<Result<Infallible, sqlx::Error>> for ApiResult
{
    fn from_residual(r: Result<Infallible, sqlx::Error>) -> Self
    {
        match r {
            Err(e) => {
                let (status, error_type, simple_msg) = match e {
                    // sqlx::Error::Configuration(error) => todo!(),
                    // sqlx::Error::InvalidArgument(_) => todo!(),
                    // sqlx::Error::Database(database_error) => todo!(),
                    // sqlx::Error::Io(error) => todo!(),
                    // sqlx::Error::Tls(error) => todo!(),
                    // sqlx::Error::Protocol(_) => todo!(),
                    sqlx::Error::RowNotFound => (
                        StatusCode::BAD_REQUEST,
                        ErrorTypes::ResourceNotFound,
                        "The requested resource could not be found.",
                    ),
                    _ => (
                        // sqlx::Error::TypeNotFound { type_name } => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ErrorTypes::InternalServerError,
                        "Unknown issue was encountered. It has been reported internally.",
                    ),
                    // sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                    //     todo!()
                    // }
                    // sqlx::Error::ColumnNotFound(_) => todo!(),
                    // sqlx::Error::ColumnDecode { index, source } => todo!(),
                    // sqlx::Error::Encode(error) => todo!(),
                    // sqlx::Error::Decode(error) => todo!(),
                    // sqlx::Error::AnyDriverError(error) => todo!(),
                    // sqlx::Error::PoolTimedOut => todo!(),
                    // sqlx::Error::PoolClosed => todo!(),
                    // sqlx::Error::WorkerCrashed => todo!(),
                    // sqlx::Error::Migrate(migrate_error) => todo!(),
                    // sqlx::Error::InvalidSavePointStatement => todo!(),
                    // sqlx::Error::BeginFailed => todo!(),
                    // _ => todo!(),
                };
                ApiResult(Err(ErrorResponse {
                    error_type,
                    status,
                    message: simple_msg.to_string(),
                }))
            }
            Ok(i) => match i {}, // unreachable
        }
    }
}

impl IntoResponse for ApiResult
{
    fn into_response(self) -> axum::response::Response
    {
        match self.0 {
            Ok(ok) => {
                let response = axum::response::Response::builder()
                    .status(ok.status)
                    .body(axum::body::Body::new(
                        json!({
                            "ok": true,
                            "data": ok.data
                        })
                        .to_string(),
                    ))
                    .unwrap();
                response
            }
            Err(err) => {
                let response = axum::response::Response::builder()
                    .status(err.status)
                    .body(axum::body::Body::new(
                        json!({
                            "ok": false,
                            "data": None::<Option<()>>,
                            "message": err.message,
                            "type": err.error_type.to_string()
                        })
                        .to_string(),
                    ))
                    .unwrap();
                response
            }
        }
    }
}

impl From<serde_json::Value> for ApiResult
{
    fn from(value: serde_json::Value) -> Self
    {
        ApiResult(Ok::<OkResponse, ErrorResponse>(OkResponse {
            status: StatusCode::OK,
            data: value,
        }))
    }
}
