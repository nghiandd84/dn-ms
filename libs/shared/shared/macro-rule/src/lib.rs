#[macro_export]
macro_rules! set_if_some {
    ($field:expr, $value:expr) => {
        if let Some(val) = $value {
            $field = Set(val);
        }
    };
}
