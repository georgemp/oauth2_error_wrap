use {failure::Fail, oauth2::ErrorResponse};

#[derive(Debug)]
pub enum Error<RE, T>
where
    RE: Fail,
    T: ErrorResponse + 'static,
{
    OAuth2RequestToken(oauth2::RequestTokenError<RE, T>),
}

impl<RE, T> From<oauth2::RequestTokenError<RE, T>> for Error<RE, T>
where
    RE: Fail,
    T: ErrorResponse + 'static,
{
    fn from(item: oauth2::RequestTokenError<RE, T>) -> Error<RE, T> {
        Error::OAuth2RequestToken(item)
    }
}
