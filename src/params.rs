use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use iron::prelude::*;
use iron::status;
use iron::Url;

#[derive(Debug)]
pub enum ParamsError {
    MissingParams,
    MissingContent,
    MissingLength,
    BadLength,
}

impl fmt::Display for ParamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParamsError {
    fn description(&self) -> &str {
        match self {
            &ParamsError::MissingParams => "params are missing",
            &ParamsError::MissingContent => "str is missing",
            &ParamsError::MissingLength => "missing length",
            &ParamsError::BadLength => "unable to parse len",
        }
    }
}

impl From<ParamsError> for IronError {
    fn from(error: ParamsError) -> IronError {
        let response = Response::with((status::BadRequest, error.description()));
        IronError {
            error: box error,
            response: response,
        }
    }
}

pub fn read_params(url: &Url) -> Result<(String, usize, char), ParamsError> {
    let query = url.clone().into_generic_url();
    match query.query_pairs() {
        None => Err(ParamsError::MissingParams),
        Some(query_params) => {
            let key_value_pairs: HashMap<_, _> = query_params.into_iter().collect();

            Ok((
                key_value_pairs.get("str")
                    .ok_or(ParamsError::MissingContent)
                    .map(|content| content.to_owned())?,

                key_value_pairs.get("len")
                    .ok_or(ParamsError::MissingLength)
                    .and_then(|length| length.parse().map_err(|_| ParamsError::BadLength))?,

                key_value_pairs.get("char")
                    .and_then(|content| content.chars().nth(0))
                    .unwrap_or(' '),
            ))
        }
    }
}
