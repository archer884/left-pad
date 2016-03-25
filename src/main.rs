#![feature(box_syntax, question_mark)]

extern crate iron;
#[macro_use] extern crate router;
extern crate rustc_serialize;

mod params;

use std::iter;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct PaddingResponse {
    original: String,
    padded: String,
    length: usize,
}

fn main() {
    Iron::new(router! {
        get "/" => help,
        get "/api/v1/pad" => pad,
        get "/api/v2/left" => left,
        get "/api/v2/right" => right,
    }).http("localhost:1337").unwrap();
}

fn help(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "curl <server>/api/v1/pad?str=John%20Smith&len=30&char=%20")))
}

fn pad(request: &mut Request) -> IronResult<Response> {
    left(request)
}

fn left(request: &mut Request) -> IronResult<Response> {
    let (content, length, padding_char) = params::read_params(&request.url)?;
    Ok(Response::with((
        status::Ok,
        json::encode(&pad_left(&content, length, padding_char)).unwrap())
    ))
}

fn right(request: &mut Request) -> IronResult<Response> {
    let (content, length, padding_char) = params::read_params(&request.url)?;
    Ok(Response::with((
        status::Ok,
        json::encode(&pad_right(&content, length, padding_char)).unwrap())
    ))
}

fn pad_right(content: &str, length: usize, padding_char: char) -> PaddingResponse {
    let padding = padding(content, length, padding_char);
    let padded_content = format!("{}{}", content, padding);

    PaddingResponse {
        original: content.to_owned(),
        padded: padded_content,
        length: length,
    }
}

fn pad_left(content: &str, length: usize, padding_char: char) -> PaddingResponse {
    let padding = padding(content, length, padding_char);
    let padded_content = format!("{}{}", padding, content);

    PaddingResponse {
        original: content.to_owned(),
        padded: padded_content,
        length: length,
    }
}

fn padding(content: &str, length: usize, padding_char: char) -> String {
    iter::repeat(padding_char).take(length - content.chars().count()).collect()
}
