use snafu::{Location, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum RustError {
    #[snafu(display("Raw Pointer is null, detail: {}", detail))]
    NullPointer {
        detail: String,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Failed to convert UTF-8, detail: {}", detail))]
    ConvertUtf8 {
        detail: String,
        source: std::str::Utf8Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Client error"))]
    Client {
        source: crate::error::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Join task"))]
    TaskJoin {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
}

pub type RustResult<T> = std::result::Result<T, RustError>;

#[repr(C)]
pub enum StatusCode {
    Success = 0,
    Err = 1,
}
