use rubrum::Sign;

/// Render a placement label template using the canonical token set.
///
/// Supported tokens:
/// - `{deg}`: degree within the sign (0..29)
/// - `{min}`: minutes (00..59)
/// - `{sec}`: seconds (00..59)
/// - `{sign}`: unicode sign glyph (e.g. ♈)
/// - `{sign_glyph}`: alias for `{sign}` (backends may treat a segment that is exactly
///   `{sign_glyph}` specially, e.g. rendering it as an SVG sprite `<use>`)
/// - `{dms}`: degree/minute/second formatted via rubrum (e.g. `12°34′56″`)
///
/// Notes:
/// - This is a plain string replacement helper; it does not interpret conditionals.
/// - Unknown tokens are left as-is.
pub fn render_placement_label_template(
    template: &str,
    sign: Sign,
    deg: i32,
    min: i32,
    sec: i32,
) -> String {
    let deg_str = format!("{deg}");
    let min_str = format!("{min:02}");
    let sec_str = format!("{sec:02}");

    let dms = rubrum::format_degrees_minutes_seconds_str(deg as f64, min as f64, sec as f64);

    template
        .replace("{deg}", &deg_str)
        .replace("{min}", &min_str)
        .replace("{sec}", &sec_str)
        .replace("{sign}", &sign.symbol_text())
        .replace("{sign_glyph}", &sign.symbol_text())
        .replace("{dms}", &dms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_tokens() {
        let out = render_placement_label_template("{sign} {deg} {min} {sec}", Sign::Aries, 1, 2, 3);
        assert_eq!(out, "♈ 1 02 03");
    }

    #[test]
    fn leaves_unknown_tokens_intact() {
        let out = render_placement_label_template("x={foo} {deg}", Sign::Taurus, 12, 0, 0);
        assert_eq!(out, "x={foo} 12");
    }

    #[test]
    fn renders_dms_token() {
        let out = render_placement_label_template("{dms}", Sign::Gemini, 12, 34, 56);
        // rubrum formatting uses its own degree/min/sec glyphs; just assert a stable substring.
        assert!(out.contains("12"));
        assert!(out.contains("34"));
        assert!(out.contains("56"));
    }
}
