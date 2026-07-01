use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UnresolvedValue {
    String(String),
    Number(f64),
    Ref { r#ref: String },
    Url { url: String },
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PresetOrigin {
    Theme,
    Custom,
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BasePreset {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub name: String,
    pub slug: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Gradient {
    pub name: String,
    pub slug: String,
    pub gradient: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Duotone {
    pub name: String,
    pub slug: String,
    pub colors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
    pub name: String,
    pub slug: PresetOrigin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradients: Option<Vec<Gradient>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duotones: Option<Vec<Duotone>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<UnresolvedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<UnresolvedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_size: Option<UnresolvedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_position: Option<UnresolvedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_repeat: Option<UnresolvedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_attachment: Option<UnresolvedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_blend_mode: Option<UnresolvedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<UnresolvedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum FluidTypographyConfig {
    Bool(bool),
    Config {
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypographyPreset {
    pub name: String,
    pub slug: String,
    pub size: Option<Value>, // string | number | null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid: Option<FluidTypographyConfig>,
}

pub type FontSizePreset = TypographyPreset;
pub type FontSize = FontSizePreset;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FontFace {
    pub font_family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_stretch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<Value>, // string | string[]
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FontFamilyPreset {
    pub name: String,
    pub slug: String,
    pub font_family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_face: Option<Vec<FontFace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum FluidTypographySettings {
    Bool(bool),
    Settings {
        #[serde(skip_serializing_if = "Option::is_none")]
        max_viewport_width: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_font_size: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_viewport_width: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypographySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid: Option<FluidTypographySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_sizes: Option<Value>, // array or object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_families: Option<HashMap<String, Vec<FontFamilyPreset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_font_sizes: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LayoutSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SpacingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_gap: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStylesSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_root_padding_aware_alignments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<TypographySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<SpacingSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStylesStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<BackgroundStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStylesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<GlobalStylesSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<GlobalStylesStyles>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StyleVariation {
    #[serde(flatten)]
    pub config: GlobalStylesConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
