use std::collections::HashMap;

use anyhow::Result;
use reqwest::{Client, Url};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Segment {
    #[serde(rename = "start")]
    first_led: u32,
    #[serde(rename = "stop")]
    first_led_nonexclusive: u32,
    #[serde(rename = "len")]
    length: u32,
    #[serde(rename = "fx")]
    effect_id: u32,
    /// colors is a vector of up to 3 elements.
    /// each element is a vector of 3 or 4 bytes. RGB(W)
    #[serde(rename = "col")]
    colors: Vec<Vec<u8>>,
    #[serde(rename = "sx")]
    relative_speed: u8,
    #[serde(rename = "ix")]
    intensity: u8,
    #[serde(rename = "bri")]
    brightness: u8,
    #[serde(rename = "rev")]
    reversed: bool,
}

#[derive(Debug, Deserialize)]
pub struct WledInfo {
    on: bool,
    #[serde(rename = "bri")]
    brightness: u8,
    #[serde(rename = "seg")]
    segments: Vec<Segment>,
}

pub struct Wled {
    url: Url,
    client: Client,
    effects: Option<Vec<String>>,
    effect_lookup: Option<HashMap<String, u32>>,
}

impl Wled {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Wled {
            url: url.parse()?,
            client: Client::new(),
            effects: None,
            effect_lookup: None,
        })
    }

    pub async fn get_info(&self) -> Result<WledInfo> {
        let mut path = self.url.clone();
        path.set_path("/json/state");
        Ok(self.client.get(path).send().await?.json().await?)
    }

    pub async fn get_effects(&self) -> Result<Vec<String>> {
        let mut path = self.url.clone();
        path.set_path("/json/effects");
        Ok(self.client.get(path).send().await?.json().await?)
    }
}
