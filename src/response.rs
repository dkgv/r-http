use std::collections::BTreeMap;

pub struct Response {
    pub properties: BTreeMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn make(status: i16, body: &str) -> Response {
        let mut properties: BTreeMap<String, String> = BTreeMap::new();

        properties.insert(
            String::from("Status"),
            format!("{} {}", status, http_message(status)),
        );
        properties.insert(
            String::from("Content-Type"),
            String::from("text/html; charset=UTF-8"),
        );
        Response {
            properties: properties,
            body: body.to_string(),
        }
    }

    pub fn ok(body: &str) -> Response {
        Response::make(200, body)
    }

    pub fn add_property(mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
}

fn http_message(code: i16) -> String {
    let message = match code {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        305 => "Use Proxy",
        306 => "Switch Proxy",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        421 => "Misdirected Request",
        422 => "Unprocessable Entity",
        423 => "Locked",
        424 => "Failed Dependency",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        _ => "OK",
    };
    String::from(message)
}
