use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RayMessage {
    Log(RayLog),
    Text(RayText),
    Color(RayColor),
    HTML(RayHtml),
    ClearAll(RayClearAll),
    Confetti(RayConfetti),
    Charles(RayCharles),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RayMessageType {
    Log,
    Text,
    HTML,
    ClearAll,
    Confetti,
    Charles,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RayContentType {
    Log,
    Custom,
    Color,
    ClearAll,
    Confetti,
}

impl RayContentType {
    pub fn to_string(&self) -> String {
        match self {
            RayContentType::Log => "log".to_string(),
            RayContentType::Custom => "custom".to_string(),
            RayContentType::Color => "color".to_string(),
            RayContentType::ClearAll => "clear_all".to_string(),
            RayContentType::Confetti => "confetti".to_string(),
        }
    }
}

// https://github.com/spatie/ray/blob/main/src/Payloads/LogPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayLog {
    pub label: RayMessageType,
    pub values: Vec<String>,
}

impl RayLog {
    pub fn get_type() -> String {
        RayContentType::Log.to_string()
    }
}

// https://github.com/spatie/ray/blob/main/src/Payloads/TextPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayText {
    pub label: RayMessageType,
    pub content: String,
}

impl RayText {
    pub fn get_type() -> String {
        RayContentType::Custom.to_string()
    }
}

// https://github.com/spatie/ray/blob/main/src/Payloads/ColorPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayColor {
    pub color: RayColors,
}

impl RayColor {
    pub fn get_type() -> String {
        RayContentType::Color.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RayColors {
    Green(String),
    Orange(String),
    Red(String),
    Purple(String),
    Blue(String),
    Gray(String),
}

impl RayColors {
    pub fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "green" => RayColors::Green("green".to_string()),
            "orange" => RayColors::Orange("orange".to_string()),
            "red" => RayColors::Red("red".to_string()),
            "purple" => RayColors::Purple("purple".to_string()),
            "blue" => RayColors::Blue("blue".to_string()),
            "gray" => RayColors::Gray("gray".to_string()),
            "grey" => RayColors::Gray("gray".to_string()), // In case someone spells it the right way
            _ => RayColors::Gray("gray".to_string()),
        }
    }
}

// https://github.com/spatie/ray/blob/main/src/Payloads/HtmlPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayHtml {
    pub label: RayMessageType,
    pub content: String,
}

impl RayHtml {
    pub fn get_type() -> String {
        RayContentType::Custom.to_string()
    }
}

// https://github.com/spatie/ray/blob/main/src/Payloads/ClearAllPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayClearAll {
    pub label: RayMessageType,
}

impl RayClearAll {
    pub fn get_type() -> String {
        RayContentType::ClearAll.to_string()
    }
}

// https://github.com/spatie/ray/blob/main/src/Payloads/ConfettiPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayConfetti {
    pub label: RayMessageType,
}

impl RayConfetti {
    pub fn get_type() -> String {
        RayContentType::Confetti.to_string()
    }
}

// https://github.com/spatie/ray/blob/main/src/Ray.php#L498
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayCharles {
    pub content: String,
}

impl RayCharles {
    pub fn get_type() -> String {
        RayContentType::Custom.to_string()
    }
}
