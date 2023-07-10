use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::path::Path;

use anyhow::anyhow;
use serde_json::Value;

// it would have been SO much easier to just manually write these impls
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn bad_json() -> anyhow::Error {
        anyhow!("bad material_data.json data")
    }

    let output = Path::new(&env::var("OUT_DIR")?).join("hue_impls.rs");

    let material_data: Value = serde_json::from_str(include_str!("material_data.json"))?;

    let shades = &material_data["Red"]
        .as_object()
        .ok_or_else(bad_json)?
        .keys()
        .collect::<Vec<_>>();

    let colors = &material_data
        .as_object()
        .ok_or_else(bad_json)?
        .iter()
        .map(|(k, v)| (k, v.as_object().unwrap())) // todo no unwrap
        .collect::<HashMap<_, _>>();

    let mut generated = String::new();
    for shade in shades {
        let mut match_arms = String::new();
        for (color, values) in colors.iter() {
            let Some(hex) = values.get(*shade) else { continue };
            let hex = hex.as_str().ok_or_else(bad_json)?.strip_prefix('#').ok_or_else(bad_json)?;
            writeln!(match_arms, r#"Hue::{color} => 0x{},"#, hex)?;
        }
        writeln!(match_arms, r#"_ => 0x0"#)?;

        let shade_name = if shade.starts_with(|c: char| c.is_ascii_digit()) {
            format!("S{shade}")
        } else {
            shade.to_ascii_uppercase()
        };
        writeln!(
            generated,
            r#"
            impl From<Hue<{shade_name}>> for Color {{
                fn from(value: Hue<{shade_name}>) -> Self {{
                    let hex = match value {{
                        {match_arms}
                    }};
                    Color {{ hex }};
                }}
            }}
        "#
        )?;
    }

    let generated_mod = format!(r#"
        #[doc(hidden)]
        pub mod impls {{
            use crate::hue::Hue;
            use crate::color::Color;
            use crate::shade::*;

            {generated}
        }}
    "#);

    std::fs::write(output, generated_mod)?;

    Ok(())
}
