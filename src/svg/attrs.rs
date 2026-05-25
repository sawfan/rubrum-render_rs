/// Replace an existing attribute value in a tag string, or insert the attribute if missing.
///
/// This is intended for small, controlled rewrites of SVG root tags (e.g. `width`, `height`,
/// `viewBox`).
///
/// Notes:
/// - This is not a full XML parser.
/// - The `tag` is expected to contain a closing `>`.
pub fn set_or_replace_svg_attr(tag: &str, name: &str, value: &str) -> String {
    let needle = format!("{name}=\"");

    // Replace an existing attribute value.
    if let Some(attr_start) = tag.find(&needle) {
        let value_start = attr_start + needle.len();
        if let Some(end_rel) = tag[value_start..].find('"') {
            let value_end = value_start + end_rel;
            let mut out = tag.to_owned();
            out.replace_range(value_start..value_end, value);
            return out;
        }
    }

    // Insert if missing.
    if let Some(insert_pos) = tag.rfind('>') {
        let mut out = tag.to_owned();
        let insert = format!(" {name}=\"{value}\"");
        out.insert_str(insert_pos, &insert);
        return out;
    }

    tag.to_owned()
}

/// Minimal attribute extractor for tags that contain attributes like `key="..."`.
///
/// This is intentionally tiny and assumes a straightforward attribute encoding.
pub fn parse_svg_attr(tag: &str, key: &str) -> Option<String> {
    let needle = format!("{key}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = start + tag[start..].find('"')?;
    Some(tag[start..end].to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_attr() {
        let out = set_or_replace_svg_attr("<svg width=\"1\" height=\"2\">", "width", "99");
        assert_eq!(out, "<svg width=\"99\" height=\"2\">");
    }

    #[test]
    fn inserts_attr() {
        let out = set_or_replace_svg_attr("<svg>", "viewBox", "0 0 10 10");
        assert_eq!(out, "<svg viewBox=\"0 0 10 10\">");
    }

    #[test]
    fn parses_attr() {
        assert_eq!(
            parse_svg_attr("<svg width=\"10\" viewBox=\"0 0 10 10\">", "viewBox").as_deref(),
            Some("0 0 10 10")
        );
    }
}
