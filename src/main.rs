/// Environment describing a CGI request.

mod template;

use cgi::{Request, Response, handle, html_response, string_response};
use cgi::http::StatusCode;
use failure::Error;
use horrorshow::*;

fn main() {
    handle(|request| {
        match handle_request(request) {
            Ok(response) => response,
            Err(error) => handle_error(error),
        }
    })
}

fn handle_request(request: Request) -> Result<Response, Error> {
    let response = html!(
        p {
            b : "Url: ";
            : format!("{:?}", request.uri());
        }
        ul {
            @for (key, val) in request.headers() {
                li {
                    b {
                        : key.to_string();
                        : ": ";
                    }
                    : val.to_str();
                }
            }
        }
        h1 : "Body";
        pre : format!("{:#?}", request.body());
    );

    Ok(template::render_page(template::global_wrapper("Test page".to_owned(), response)))
}

fn handle_error(error: Error) -> Response {
    string_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Error: {:?} ", error.name())
    )
}
