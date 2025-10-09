use serde::Serialize;
use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};

#[derive(Serialize, Debug, PartialEq)]
pub struct BaseResp<T>
where
    T: Serialize,
{
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> BaseResp<T>
where
    T: Serialize,
{
    pub fn new(code: i32, msg: String, data: T) -> Self {
        Self { code, msg, data }
    }
}

impl<T> Responder for BaseResp<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = match serde_json::to_string(&self) {
            Ok(b) => b,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Serialize, Debug, PartialEq)]
    struct TestData {
        value: i32,
    }

    #[test]
    fn test_base_resp_new_and_serialize() {
        let data = TestData { value: 42 };
        let resp = BaseResp::new(0, "成功".to_string(), data);
        let expected_json = json!({
            "code": 0,
            "msg": "成功",
            "data": { "value": 42 }
        });
        let serialized = serde_json::to_value(&resp).unwrap();
        assert_eq!(serialized, expected_json);
    }
}

