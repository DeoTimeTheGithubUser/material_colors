use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::path::Path;

use anyhow::anyhow;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const OUTPUT_FILE: &str = "hue_impls.rs";

    fn bad_json() -> anyhow::Error {
        anyhow!("bad material_data.json data")
    }

    let output = Path::new(&env::var("OUT_DIR")?).join(OUTPUT_FILE);
    let material_data: Value = serde_json::from_str(include_str!("material_data.json"))?;

    let shades = &material_data["Red"]
        .as_object()
        .ok_or_else(bad_json)?
        .keys()
        .collect::<Vec<_>>();

    let hues = &material_data
        .as_object()
        .ok_or_else(bad_json)?
        .iter()
        .map(|(k, v)| (k, v.as_object().unwrap())) // todo no unwrap
        .collect::<HashMap<_, _>>();

    let mut generated = String::new();
    for (hue, values) in hues.iter() {
        let mut consts = String::new();
        for shade in shades {
            let shade_name = if shade.starts_with(|c: char| c.is_ascii_digit()) {
                format!("S{shade}")
            } else {
                shade.to_ascii_uppercase()
            };

            let hex = values
                .get(*shade)
                .and_then(|h| h.as_str())
                .unwrap_or("#0")
                .strip_prefix('#')
                .ok_or_else(bad_json)?;

            writeln!(
                consts,
                r#"const {shade_name}: Color = Color::from_hex(0x{hex});"#
            )?;
        }

        writeln!(
            generated,
            r#"

                pub struct {hue};

                impl Hue for {hue} {{
                    {consts}
                }}

                lazy_static! {{
                    static ref _{hue}S500: Color = {hue}::S500;
                }}

                impl Deref for {hue} {{
                    type Target = Color;

                    fn deref(&self) -> &'static Self::Target {{
                        &_{hue}S500
                    }}
                }}

                impl private::Sealed for {hue} {{}}
            "#
        )?;
    }

    let generated_mod = format!(
        r#"
        #[allow(non_upper_case_globals)]
        mod impls {{
            use super::private;
            use std::ops::Deref;
            use crate::hue::Hue;
            use crate::color::Color;
            use lazy_static::lazy_static;

            {generated}
        }}

        pub use impls::*;


    "#
    );

    let file = syn::parse_file(&generated_mod)?;
    std::fs::write(output, prettyplease::unparse(&file))?;

    println!("cargo:rustc-env=GENERATED_HUES={OUTPUT_FILE}");
    println!("cargo:rustc-cfg=generated_hues");
    Ok(())
}
