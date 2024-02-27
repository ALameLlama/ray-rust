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
        let ray = Ray::new();

        ray
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

#[macro_export]
macro_rules! rd {
    () => {{
        let mut ray = Ray::new();

        ray.die(1);

        ray
    }};
    ($($arg:expr),*) => {{
        let mut ray = Ray::new();
        let mut vec = Vec::new();

        $(vec.push(format!("{:#?}", $arg));)*

        ray.log(vec);

        ray.die(1);

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
    is_enabled: bool,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            request: RayPayload {
                uuid: uuid::Uuid::new_v4().to_string(),
                payloads: vec![],
                meta: RayMeta::new(),
            },
            is_enabled: true,
        }
    }

    pub fn send(&mut self) -> &mut Self {
        if !self.is_enabled {
            return self;
        }

        let request = self.request.clone();

        let client = reqwest::blocking::Client::new();

        let _ = client.post("http://localhost:23517").json(&request).send();

        self
    }

    pub fn die(&mut self, status: i32) {
        panic!("exited with code {}", status);

        // TODO: I think we need to use process::exit here to actually exit the process since this
        // doesn't stop threaded work but I can't see a nice way to test this. I'm going to leave it
        // std::process::exit(status);
    }

    pub fn clear_all(&mut self) -> &mut Self {
        let message = RayMessage::ClearAll(RayClearAll {
            label: RayMessageType::ClearAll,
        });

        let content = RayContent {
            content_type: RayClearAll::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }

    pub fn new_screen(&mut self, name: Option<&str>) -> &mut Self {
        let message = RayMessage::NewScreen(RayNewScreen {
            label: RayMessageType::NewScreen,
            name: name.unwrap_or("").to_string(),
        });

        let content = RayContent {
            content_type: RayNewScreen::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }

    pub fn clear_screen(&mut self) -> &mut Self {
        self.new_screen(None)
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

    pub fn html(&mut self, value: &str) -> &mut Self {
        let message = RayMessage::HTML(RayHtml {
            label: RayMessageType::HTML,
            content: value.to_string(),
        });

        let content = RayContent {
            content_type: RayHtml::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }

    pub fn confetti(&mut self) -> &mut Self {
        let message = RayMessage::Confetti(RayConfetti {
            label: RayMessageType::Confetti,
        });

        let content = RayContent {
            content_type: RayConfetti::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }

    pub fn charles(&mut self) -> &mut Self {
        let message = RayMessage::Charles(RayCharles {
            content: "🎶 🎹 🎷 🕺".to_string(),
        });

        let content = RayContent {
            content_type: RayCharles::get_type(),
            origin: RayOrigin::new(),
            content: message,
        };

        self.request.payloads.push(content);

        self.send()
    }

    pub fn count(&mut self, _name: &str) -> &mut Self {
        // create a new counter hashmap with the name?
        // increment the counter hashmap with the name?
        // return a custom message with "called name x times"

        todo!();
    }

    pub fn counter_value(&mut self) -> &mut Self {
        todo!();
    }

    pub fn clear_counters(&mut self) -> &mut Self {
        todo!();
    }

    pub fn disable(&mut self) -> &mut Self {
        self.is_enabled = false;

        self
    }

    pub fn disabled(&mut self) -> bool {
        !self.is_enabled
    }

    pub fn enable(&mut self) -> &mut Self {
        self.is_enabled = true;

        self
    }

    pub fn enabled(&mut self) -> bool {
        self.is_enabled
    }

    pub fn file(&mut self) -> &mut Self {
        todo!();
    }

    pub fn gray(&mut self) -> &mut Self {
        todo!();
    }

    pub fn green(&mut self) -> &mut Self {
        todo!();
    }

    pub fn hide(&mut self) -> &mut Self {
        todo!();
    }

    pub fn hide_app(&mut self) -> &mut Self {
        todo!();
    }

    pub fn image(&mut self) -> &mut Self {
        todo!();
    }

    // I'm not sure if this is possible in Rust
    pub fn r#if(&mut self) -> &mut Self {
        todo!();
    }

    pub fn json(&mut self) -> &mut Self {
        todo!();
    }

    pub fn label(&mut self) -> &mut Self {
        todo!();
    }

    pub fn large(&mut self) -> &mut Self {
        todo!();
    }

    pub fn limit(&mut self) -> &mut Self {
        todo!();
    }

    pub fn link(&mut self) -> &mut Self {
        todo!();
    }

    pub fn measure(&mut self) -> &mut Self {
        todo!();
    }

    pub fn notify(&mut self) -> &mut Self {
        todo!();
    }

    pub fn orange(&mut self) -> &mut Self {
        todo!();
    }

    pub fn pass(&mut self) -> &mut Self {
        todo!();
    }

    pub fn pause(&mut self) -> &mut Self {
        todo!();
    }

    pub fn info(&mut self) -> &mut Self {
        todo!();
    }

    pub fn purple(&mut self) -> &mut Self {
        todo!();
    }

    // TODO: This has 3 functions max, per_second and clear
    pub fn rate_limiter(&mut self) -> &mut Self {
        todo!();
    }

    pub fn red(&mut self) -> &mut Self {
        todo!();
    }

    pub fn separator(&mut self) -> &mut Self {
        todo!();
    }

    pub fn show_app(&mut self) -> &mut Self {
        todo!();
    }

    pub fn small(&mut self) -> &mut Self {
        todo!();
    }

    pub fn table(&mut self) -> &mut Self {
        todo!();
    }

    pub fn to_json(&mut self) -> &mut Self {
        todo!();
    }

    pub fn trace(&mut self) -> &mut Self {
        todo!();
    }

    pub fn url(&mut self) -> &mut Self {
        todo!();
    }

    pub fn xml(&mut self) -> &mut Self {
        todo!();
    }
}
