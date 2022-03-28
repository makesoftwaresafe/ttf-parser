#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(face) = ttf_parser::Face::from_slice(&data, 0) {
        let family_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == ttf_parser::name_id::FULL_NAME)
            .and_then(|name| name.to_string());

        let post_script_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME)
            .and_then(|name| name.to_string());

        println!("Family name: {:?}", family_name);
        println!("PostScript name: {:?}", post_script_name);
        println!("Units per EM: {:?}", face.units_per_em());
        println!("Ascender: {}", face.ascender());
        println!("Descender: {}", face.descender());
        println!("Line gap: {}", face.line_gap());
        println!("Global bbox: {:?}", face.global_bounding_box());
        println!("Number of glyphs: {}", face.number_of_glyphs());
        println!("Underline: {:?}", face.underline_metrics());
        println!("X height: {:?}", face.x_height());
        println!("Weight: {:?}", face.weight());
        println!("Width: {:?}", face.width());
        println!("Regular: {}", face.is_regular());
        println!("Italic: {}", face.is_italic());
        println!("Bold: {}", face.is_bold());
        println!("Oblique: {}", face.is_oblique());
        println!("Strikeout: {:?}", face.strikeout_metrics());
        println!("Subscript: {:?}", face.subscript_metrics());
        println!("Superscript: {:?}", face.superscript_metrics());
        println!("Variable: {:?}", face.is_variable());

        #[cfg(feature = "variable-fonts")]
        {
            if face.is_variable() {
                println!("Variation axes:");
                for axis in face.variation_axes() {
                    println!(
                        "  {} {}..{}, default {}",
                        axis.tag, axis.min_value, axis.max_value, axis.def_value
                    );
                }
            }
        }
    }
});
