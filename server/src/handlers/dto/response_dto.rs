use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDto<T> {
    message: String,
    data: T,
}

impl<T> ResponseDto<T>
where
    T: serde::Serialize,
{
    pub fn new(message: &str, data: T) -> Self {
        ResponseDto {
            message: message.to_string(),
            data,
        }
    }
}
