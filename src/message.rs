use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RayMessage {
    Log(RayLog),
    Text(RayText),
    Color(RayColor),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RayMessageType {
    Log,
    Text,
}

// https://github.com/spatie/ray/blob/main/src/Payloads/LogPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayLog {
    pub label: RayMessageType,
    pub values: Vec<String>,
}

impl RayLog {
    pub fn get_type() -> String {
        "log".to_string()
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
        "custom".to_string()
    }
}

// https://github.com/spatie/ray/blob/main/src/Payloads/ColorPayload.php
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RayColor {
    pub color: RayColors,
}

impl RayColor {
    pub fn get_type() -> String {
        "color".to_string()
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
