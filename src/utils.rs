#[macro_export]
macro_rules! err {
    ( $kind:ident ) => {
        ::std::io::Error::from(
            ::std::io::ErrorKind::$kind
        )
    };
    ( $kind:ident, $err:expr ) => {
        ::std::io::Error::new(
            ::std::io::ErrorKind::$kind,
            $err
        )
    };
    ( $kind:ident, $fmt:expr, $( $args:tt )+ ) => {
        err!($kind, format!($fmt, $($args)+))
    }
}


pub const START: &str = "Pleased to meet you";
pub const CLOSE: &str = "closing connection";
pub const USER_UNKNOWN_COMMAND: &str = "536871187 Unknown IPC command <User defined source 1>";
pub const USER_NOT_IMPLEMENTED: &str = "536870981 Not implemented <User defined source 1>";
pub const PINENTRY_OPERATION_CANCELLED: &str = "83886179 Operation cancelled <Pinentry>";
pub const PINENTRY_NOT_CONFIRMED: &str = "83886194 Not confirmed <Pinentry>";
pub const PINENTRY_PARAMETER_ERROR: &str = "83886360 IPC parameter error <Pinentry>";
