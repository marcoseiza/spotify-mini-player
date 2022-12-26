#[macro_export]
macro_rules! unwrap_or {
    ($res: expr, $code: expr) => {
        match $res {
            Ok(v) => v,
            Err(_) => $code,
        }
    };
}
