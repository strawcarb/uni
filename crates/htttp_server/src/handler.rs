use http::http_request::Resource;
use http::{http_request::HttpRequest, http_response::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(request: &HttpRequest) -> HttpResponse;
    fn load_file(path: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, path);
        println!("Try to load file :{}", full_path);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticFileHandler;
pub struct WebServiceHandler;
pub struct PageNotFoundHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: String,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(_request: &HttpRequest) -> HttpResponse {
        HttpResponse::new(404, None, Self::load_file("404.html"))
    }
}

impl Handler for StaticFileHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let Resource::Path(path) = &request.resource;
        let route: Vec<&str> = path.split("/").collect();
        println!("StaticFileHandler handle path: {:?}", path);
        match route[1] {
            "" => HttpResponse::new(200, None, Self::load_file("index.html")),
            "health" => HttpResponse::new(200, None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else if path.ends_with(".html") {
                        map.insert("Content-Type", "text/html");
                    } else if path.ends_with(".png") {
                        map.insert("Content-Type", "image/png");
                    } else if path.ends_with(".jpg") {
                        map.insert("Content-Type", "image/jpeg");
                    } else if path.ends_with(".gif") {
                        map.insert("Content-Type", "image/gif");
                    }
                    HttpResponse::new(200, Some(map), Some(contents))
                }
                None => {
                    println!("File not found");
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    map.insert("Content-Type", "text/html");
                    HttpResponse::new(404, Some(map), Self::load_file("404.html"))
                }
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let Resource::Path(path) = &request.resource;
        let route: Vec<&str> = path.split("/").collect();
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new(200, Some(headers), body)
            }
            _ => {
                let response = PageNotFoundHandler::handle(&request);
                response
            }
        }
    }
}
