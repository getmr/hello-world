use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum MyError {
    #[error("内部服务器错误: {0}")]
    InternalError(String),

    #[error("客户端请求错误: {0}")]
    BadClientData(String),

    #[error("请求超时: {0}")]
    Timeout(String),

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("验证错误: {0}")]
    ValidationError(String),

    #[error("未找到资源: {0}")]
    NotFound(String),

    #[error("未授权访问: {0}")]
    Unauthorized(String),

    #[error("禁止访问: {0}")]
    Forbidden(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown(String),

}

#[allow(dead_code)]
impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(serde_json::json!({
                "error": self.to_string(),
                "status": self.status_code().as_u16()
            }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData(_) => StatusCode::BAD_REQUEST,
            MyError::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
            MyError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::ValidationError(_) => StatusCode::BAD_REQUEST,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            MyError::Forbidden(_) => StatusCode::FORBIDDEN,
            MyError::InvalidHeader { .. } => StatusCode::BAD_REQUEST,
            MyError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// 为 MyError 实现一些便利方法
#[allow(dead_code)]
impl MyError {
    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self::InternalError(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadClientData(msg.into())
    }

    pub fn timeout(msg: impl Into<String>) -> Self {
        Self::Timeout(msg.into())
    }

    pub fn validation_error(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden(msg.into())
    }

    pub fn invalid_header(expected: impl Into<String>, found: impl Into<String>) -> Self {
        Self::InvalidHeader { expected: expected.into(), found: found.into() }
    }

    pub fn unknown() -> Self {
        Self::Unknown(String::new())
    }
}

// 实现 From trait 以便于错误转换
impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        Self::BadClientData(format!("JSON 解析错误: {}", err))
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        Self::InternalError(format!("IO 错误: {}", err))
    }
}
