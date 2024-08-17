use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Trace,
    Connect,
    Patch,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "HEAD" => Method::Head,
            "OPTIONS" => Method::Options,
            "TRACE" => Method::Trace,
            "CONNECT" => Method::Connect,
            "PATCH" => Method::Patch,
            _ => panic!("Invalid method"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Get;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "".to_string();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(&line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_req_header(&line);
                parsed_headers.insert(key, value);
            } else {
                parsed_body = line.to_string()
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: parsed_body,
        }
    }
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap();
    let resource = parts.next().unwrap();
    let version = parts.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_req_header(line: &str) -> (String, String) {
    let mut parts = line.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = parts.next() {
        key = k.to_string();
    }
    if let Some(v) = parts.next() {
        value = v.to_string();
    }
    (key, value)
}

#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_message: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from_str() {
        assert_eq!(Method::from("GET"), Method::Get);
        assert_eq!(Method::from("POST"), Method::Post);
        assert_eq!(Method::from("PUT"), Method::Put);
        assert_eq!(Method::from("DELETE"), Method::Delete);
        assert_eq!(Method::from("HEAD"), Method::Head);
        assert_eq!(Method::from("OPTIONS"), Method::Options);
        assert_eq!(Method::from("TRACE"), Method::Trace);
        assert_eq!(Method::from("CONNECT"), Method::Connect);
        assert_eq!(Method::from("PATCH"), Method::Patch);
    }
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_from_str() {
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("HTTP/3.0"), Version::Uninitialized);
    }
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /hello HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\n");
        let mut headers_expceted = HashMap::new();
        headers_expceted.insert("Host".into(), " localhost".into());
        headers_expceted.insert("Accept".into(), " */*".into());
        headers_expceted.insert("User-Agent".into(), " curl/7.54.0".into());

        let req = HttpRequest::from(s);
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.resource, Resource::Path("/hello".into()));
        assert_eq!(req.headers, headers_expceted);
        assert_eq!(req.body, "");
    }
}
