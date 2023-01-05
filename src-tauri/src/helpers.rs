use serde::Serializer;

#[macro_export]
macro_rules! unwrap_or {
    ($res: expr, $code: expr) => {
        match $res {
            Ok(v) => v,
            Err(_) => $code,
        }
    };
}

pub fn to_string<S, E>(error: &E, s: S) -> Result<S::Ok, S::Error>
where
    E: std::error::Error + ToString,
    S: Serializer,
{
    s.serialize_str(error.to_string().as_str())
}
