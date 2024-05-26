pub enum ErrorsType {
    InternalServerError,
}

pub struct Errors {
    pub error_type: ErrorsType,
    pub message: String,
}

impl Errors {
    pub fn new(error_type: ErrorsType, message: String) -> Errors {
        Errors {
            error_type,
            message,
        }
    }
}
