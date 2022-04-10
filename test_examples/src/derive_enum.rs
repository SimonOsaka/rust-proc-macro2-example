use test_proc_macros::ErrorMessage;

#[derive(ErrorMessage)]
enum LoginError {
    #[error_message("password is not correct.")]
    Password,
    #[error_message("user name is not exist.")]
    Username,
    #[error_message("token.")]
    Token { token: String },
    #[error_message("expired.")]
    Expired(String),
}
