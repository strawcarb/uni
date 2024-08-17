use std::collections::HashMap;

use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: u16,
    status_message: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: 200,
            status_message: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse<'a>) -> String {
        let res = response.clone();
        format!(
            "{} {} {}\r\nContent-Length:{}\r\n{}\r\n{}",
            res.version(),
            res.status_code(),
            res.status_message(),
            response.body.unwrap().len(),
            res.headers(),
            res.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: u16,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != 200 {
            response.status_code = status_code;
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_message = match response.status_code {
            200 => "OK",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown Error",
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(stream, "{}", response_string);
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> u16 {
        self.status_code
    }

    fn status_message(&self) -> &str {
        self.status_message
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut headers: String = String::from("");
        for (key, value) in map {
            headers = format!("{}{}:{}\r\n", headers, key, value);
        }
        headers
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_response_creation_200() {
        let response_actual = HttpResponse::new(200, None, Some("Hello World".into()));

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: 200,
            status_message: "OK",
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            },
            body: Some("Hello World".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation_400() {
        let response_actual = HttpResponse::new(400, None, Some("Hello World".into()));

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: 400,
            status_message: "Bad Request",
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            },
            body: Some("Hello World".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation_401() {
        let response_actual = HttpResponse::new(401, None, Some("Hello World".into()));

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: 401,
            status_message: "Unauthorized",
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            },
            body: Some("Hello World".to_string()),
        };

        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: 200,
            status_message: "OK",
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            },
            body: Some("Hello World".to_string()),
        };
        let expected: String = response_expected.into();
        let actual = String::from(
            "HTTP/1.1 200 OK\r\nContent-Length:11\r\nContent-Type:text/html\r\n\r\nHello World",
        );
        assert_eq!(expected, actual);
    }
}
