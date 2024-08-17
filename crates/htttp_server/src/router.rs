use super::handler::{Handler, PageNotFoundHandler, StaticFileHandler, WebServiceHandler};
use http::{
    http_request::HttpRequest, http_request::Method, http_request::Resource
};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(request: HttpRequest, stream: &mut impl Write) -> () {
        match request.method {
            Method::Get => match &request.resource {
                Resource::Path(path) => {
                    println!("Route path: {:?}", path);
                    let route: Vec<&str> = path.split("/").collect();
                    println!("Route route: {:?}", route);
                    match route[1] {
                        "api" => {
                            let response = WebServiceHandler::handle(&request);
                            let _ = response.send_response(stream);
                        }
                        _ => {
                            let response = StaticFileHandler::handle(&request);
                            let _ = response.send_response(stream);
                        }
                    }
                }
            },
            Method::Post => {}
            _ => {
                let response = PageNotFoundHandler::handle(&request);
                let _ = response.send_response(stream);
            }
        }
    }
}
