use crate::layout::LaneSpec;
use crate::style::LaneTemplate;
use crate::theme::Theme;

/// Resolve the effective lane style for a lane.
///
/// Precedence:
/// - Start with [`LaneTemplate::default()`]
/// - Apply the referenced theme template (if any)
/// - Apply lane overrides (only fields that are `Some`)
pub fn resolve_lane_style(theme: &Theme, lane: &LaneSpec) -> LaneTemplate {
    let mut out = LaneTemplate::default();

    if let Some(template_name) = &lane.template
        && let Some(template) = theme.templates.get(template_name)
    {
        out = template.clone();
    }

    if lane.overrides.fill.is_some() {
        out.fill = lane.overrides.fill;
    }
    if lane.overrides.stroke.is_some() {
        out.stroke = lane.overrides.stroke;
    }
    if lane.overrides.stroke_width.is_some() {
        out.stroke_width = lane.overrides.stroke_width;
    }
    if lane.overrides.font_family.is_some() {
        out.font_family = lane.overrides.font_family.clone();
    }
    if lane.overrides.font_size.is_some() {
        out.font_size = lane.overrides.font_size;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_overrides_win() {
        let mut theme = Theme::default();
        theme.templates.insert(
            "t".to_owned(),
            LaneTemplate {
                fill: Some(crate::options::RgbaColor {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                stroke: None,
                stroke_width: None,
                font_family: Some("FromTemplate".to_owned()),
                font_size: Some(11.0),
            },
        );

        let lane = LaneSpec {
            template: Some("t".to_owned()),
            overrides: LaneTemplate {
                font_family: Some("FromOverride".to_owned()),
                ..Default::default()
            },
            ..Default::default()
        };

        let resolved = resolve_lane_style(&theme, &lane);
        assert_eq!(resolved.font_family.as_deref(), Some("FromOverride"));
        assert_eq!(resolved.font_size, Some(11.0));
    }
}
