use crate::options::RgbaColor;

/// Escape a string for use as XML text content (minimal, SVG-focused).
///
/// This is intentionally small and only escapes the characters we commonly encounter.
pub fn escape_xml_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Escape a string for use as a quoted XML attribute value.
///
/// This escapes `& < > " '`. (Yes, `>` isn't strictly required in attributes, but escaping it is
/// harmless and keeps symmetry with text escaping.)
pub fn escape_xml_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Convert an arbitrary key into a CSS-token-safe string.
///
/// Convention:
/// - ASCII alnum stays as-is
/// - everything else becomes '-'
/// - repeated '-' are squashed
pub fn key_to_css_token(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    let mut prev_dash = false;

    for ch in key.chars() {
        let keep = ch.is_ascii_alphanumeric() || ch == '_' || ch == '-';
        if keep {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }

    out.trim_matches('-').to_owned()
}

/// Convert a (debug) enum name like `MajorTrine` into a kebab-case token like `major-trine`.
pub fn camel_to_kebab(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// Convert a canonical key (often snake_case, sometimes qualified like `chart_point:true_node`)
/// into a CSS-token-safe string.
///
/// This is more specific than [`key_to_css_token`], keeping the output stable for Rubrum's
/// canonical keys.
pub fn canonical_key_to_css_token(key: &str) -> String {
    key.replace(['_', ':'], "-")
}

/// Convert a premultiplied rgba float color into a CSS `rgba(r,g,b,a)` string.
pub fn rgba_css(c: RgbaColor) -> String {
    let r = (c.r.clamp(0.0, 1.0) * 255.0).round() as i32;
    let g = (c.g.clamp(0.0, 1.0) * 255.0).round() as i32;
    let b = (c.b.clamp(0.0, 1.0) * 255.0).round() as i32;
    let a = c.a.clamp(0.0, 1.0);
    format!("rgba({r},{g},{b},{a})")
}

/// Like [`rgba_css`], but allows scaling alpha (commonly used by stroke alpha overrides).
pub fn rgba_css_with_alpha_mul(c: RgbaColor, alpha_mul: f64) -> String {
    let r = (c.r.clamp(0.0, 1.0) * 255.0).round() as i32;
    let g = (c.g.clamp(0.0, 1.0) * 255.0).round() as i32;
    let b = (c.b.clamp(0.0, 1.0) * 255.0).round() as i32;
    let a = (c.a.clamp(0.0, 1.0) * alpha_mul).clamp(0.0, 1.0);
    format!("rgba({r},{g},{b},{a})")
}

/// Format `stroke-dasharray="..."` given a slice of numbers.
pub fn fmt_stroke_dasharray_attr(dash: &[f64]) -> Option<String> {
    if dash.is_empty() {
        return None;
    }

    let parts = dash
        .iter()
        .map(|v| format!("{v}"))
        .collect::<Vec<_>>()
        .join(",");

    Some(format!(" stroke-dasharray=\"{parts}\""))
}

/// Format the SVG `stroke-linecap` token for a [`StrokeLineCap`].
///
/// Kept in `rubrum_render::svg` so both Cairo SVG-injection and pure-SVG backends can share
/// identical output.
pub fn fmt_stroke_linecap_attr(linecap: crate::aspects::StrokeLineCap) -> &'static str {
    match linecap {
        crate::aspects::StrokeLineCap::Butt => "butt",
        crate::aspects::StrokeLineCap::Round => "round",
        crate::aspects::StrokeLineCap::Square => "square",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_attr_escapes_quotes() {
        assert_eq!(escape_xml_attr("a\"b"), "a&quot;b");
    }

    #[test]
    fn kebab() {
        assert_eq!(camel_to_kebab("MajorTrine"), "major-trine");
    }

    #[test]
    fn css_token_squash() {
        assert_eq!(key_to_css_token("Hello, world!"), "hello-world");
    }
}
