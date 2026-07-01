use serde_json::Value;

pub mod types;

fn process_custom(prefix: &str, value: &Value, css_vars: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let new_prefix = if prefix.is_empty() {
                    format!("--wp--custom--{}", k)
                } else {
                    format!("{}--{}", prefix, k)
                };
                process_custom(&new_prefix, v, css_vars);
            }
        }
        Value::String(s) => {
            css_vars.push(format!("{}: {};", prefix, s));
        }
        Value::Number(n) => {
            css_vars.push(format!("{}: {};", prefix, n));
        }
        Value::Bool(b) => {
            css_vars.push(format!("{}: {};", prefix, b));
        }
        _ => {}
    }
}

pub fn compile_global_styles(config: &types::GlobalStylesConfig) -> String {
    let mut css_vars = Vec::new();

    if let Some(settings) = &config.settings {
        if let Some(color) = &settings.color {
            if let Some(palette) = color.get("palette").and_then(|v| v.as_array()) {
                for item in palette {
                    if let (Some(slug), Some(color_val)) = (
                        item.get("slug").and_then(|v| v.as_str()),
                        item.get("color").and_then(|v| v.as_str()),
                    ) {
                        css_vars.push(format!("--wp--preset--color--{}: {};", slug, color_val));
                    }
                }
            }
            if let Some(gradients) = color.get("gradients").and_then(|v| v.as_array()) {
                for item in gradients {
                    if let (Some(slug), Some(gradient_val)) = (
                        item.get("slug").and_then(|v| v.as_str()),
                        item.get("gradient").and_then(|v| v.as_str()),
                    ) {
                        css_vars.push(format!("--wp--preset--gradient--{}: {};", slug, gradient_val));
                    }
                }
            }
        }

        if let Some(typography) = &settings.typography {
            if let Some(font_sizes) = &typography.font_sizes {
                if let Some(arr) = font_sizes.as_array() {
                    for item in arr {
                        if let Some(slug) = item.get("slug").and_then(|v| v.as_str()) {
                            if let Some(size) = item.get("size") {
                                let size_str = match size {
                                    Value::String(s) => s.clone(),
                                    Value::Number(n) => n.to_string(),
                                    _ => continue,
                                };
                                css_vars.push(format!("--wp--preset--font-size--{}: {};", slug, size_str));
                            }
                        }
                    }
                }
            }
            
            if let Some(font_families_map) = &typography.font_families {
                for (_group, presets) in font_families_map {
                    for preset in presets {
                        css_vars.push(format!("--wp--preset--font-family--{}: {};", preset.slug, preset.font_family));
                    }
                }
            }
        }
        
        // Custom
        if let Some(custom) = &settings.custom {
            for (k, v) in custom {
                let prefix = format!("--wp--custom--{}", k);
                process_custom(&prefix, v, &mut css_vars);
            }
        }
    }

    let mut css_rules = Vec::new();

    if !css_vars.is_empty() {
        let mut vars_block = String::from("body {\n");
        for var in css_vars {
            vars_block.push_str(&format!("  {}\n", var));
        }
        vars_block.push_str("}");
        css_rules.push(vars_block);
    }

    if let Some(styles) = &config.styles {
        if let Some(css) = &styles.css {
            css_rules.push(css.clone());
        }
    }

    css_rules.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_compile_empty_styles() {
        let config = types::GlobalStylesConfig {
            version: Some(2),
            settings: None,
            styles: None,
        };
        assert_eq!(compile_global_styles(&config), "");
    }
}
