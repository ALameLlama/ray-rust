use rustc_version::version_meta;
use serde::{Deserialize, Serialize};

mod message;
use message::*;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! ray {
    // If no arguments are passed, just create a new Ray instance
    () => {{
        Ray::new()
    }};
    // If one or more arguments are passed, log them
    ($($arg:expr),*) => {{
        let mut ray = Ray::new();
        let mut vec = Vec::new();

        // Push each argument to the vector
        $(vec.push(format!("{:#?}", $arg));)*

        ray.log(vec);

        ray
    }};
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayPayload {
    uuid: String,
    payloads: Vec<RayContent>,
    meta: RayMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayContent {
    #[serde(rename = "type")] // rename the field to "type" since it's a reserved keyword
    content_type: String,
    content: RayMessage,
    origin: RayOrigin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayOrigin {
    function_name: String,
    file: String,
    line_number: u32,
    hostname: String,
}

impl RayOrigin {
    pub fn new() -> Self {
        // TODO: Get the function name, file, and line number from the macro
        // I don't think we can get the function name, file, and line number from the macro, at
        // least not easily. So we'll just set them to empty strings and 0 for now.
        let function_name = "ray".to_string();
        let file = "".to_string();
        let line_number = 0;
        let hostname = "localhost".to_string();

        Self {
            function_name,
            file,
            line_number,
            hostname,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayMeta {
    // TODO: See if we can get more useful information, edition? etc.
    // I don't even what if this shows up on ray, but I'm going to leave it here for now
    rustc_version: String,
    package_version: String,
}

impl RayMeta {
    pub fn new() -> Self {
        let rustc_version = match version_meta() {
            Ok(meta) => meta.short_version_string,
            Err(_) => "🤷".to_string(),
        };

        Self {
            rustc_version,
            package_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

pub struct Ray {
    request: RayPayload,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            request: RayPayload {
                uuid: uuid::Uuid::new_v4().to_string(),
                payloads: vec![],
                meta: RayMeta::new(),
            },
        }
    }

    pub fn send(&mut self) -> &mut Self {
        let request = self.request.clone();

        let client = reqwest::blocking::Client::new();

        let _ = client.post("http://localhost:23517").json(&request).send();

        self
    }

    pub fn log(&mut self, values: Vec<String>) -> &mut Self {
        let message = RayMessage::Log(RayLog {
            label: RayMessageType::Log,
            values,
        });

        let content = RayContent {
            content_type: RayLog::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }

    pub fn text(&mut self, value: &str) -> &mut Self {
        let message = RayMessage::Text(RayText {
            label: RayMessageType::Text,
            content: value.to_string(),
        });

        let content = RayContent {
            content_type: RayText::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }

    pub fn color(&mut self, value: &str) -> &mut Self {
        let message = RayMessage::Color(RayColor {
            color: RayColors::from(value.to_string()),
        });

        let content = RayContent {
            content_type: RayColor::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }
}
