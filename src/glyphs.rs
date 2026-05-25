use rubrum::{Angle, Body, ChartPoint, Occupant, Sign};

pub fn occupant_label(occupant: Occupant) -> String {
    match occupant {
        Occupant::Empty => String::new(),
        Occupant::Body(body) => body.symbol_text().to_string(),
        Occupant::Angle(angle) => angle.symbol_text().to_string(),
        Occupant::ChartPoint(point) => point.symbol_text().to_string(),
        Occupant::Lot(lot) => lot.symbol_text(),
    }
}

pub fn body_svg_symbol_id(body: Body) -> String {
    format!("rb-body-{}", Body::canonical_key(body))
}

pub fn chart_point_svg_symbol_id(point: ChartPoint) -> String {
    format!("rb-chart-point-{}", ChartPoint::canonical_key(point))
}

pub fn angle_svg_symbol_id(angle: Angle) -> String {
    format!("rb-angle-{}", Angle::canonical_key(angle))
}

pub fn lot_svg_symbol_id(lot: rubrum::Lot) -> String {
    format!("rb-lot-{}", rubrum::Lot::canonical_key(lot))
}

pub fn sign_svg_symbol_id(sign: Sign) -> &'static str {
    match sign {
        Sign::Aries => "rb-sign-aries",
        Sign::Taurus => "rb-sign-taurus",
        Sign::Gemini => "rb-sign-gemini",
        Sign::Cancer => "rb-sign-cancer",
        Sign::Leo => "rb-sign-leo",
        Sign::Virgo => "rb-sign-virgo",
        Sign::Libra => "rb-sign-libra",
        Sign::Scorpio => "rb-sign-scorpio",
        Sign::Sagittarius => "rb-sign-sagittarius",
        Sign::Capricorn => "rb-sign-capricorn",
        Sign::Aquarius => "rb-sign-aquarius",
        Sign::Pisces => "rb-sign-pisces",
    }
}

/// Legacy fixed filenames for the oldest body packs.
pub fn glyph_pack_body_file_name(body: Body) -> Option<&'static str> {
    match body {
        Body::Sun => Some("Sun_symbol_(fixed_width).svg"),
        Body::Moon => Some("Moon_decrescent_symbol_(fixed_width).svg"),
        Body::Mercury => Some("Mercury_symbol_(fixed_width).svg"),
        Body::Venus => Some("Venus_symbol_(fixed_width).svg"),
        Body::Mars => Some("Mars_symbol_(fixed_width).svg"),
        Body::Jupiter => Some("Jupiter_symbol_(fixed_width).svg"),
        Body::Saturn => Some("Saturn_symbol_(fixed_width).svg"),
        Body::Uranus => Some("Uranus_symbol_(fixed_width).svg"),
        Body::Neptune => Some("Neptune_symbol_(fixed_width).svg"),
        Body::Pluto => Some("Pluto_symbol_(large_orb,_fixed_width).svg"),
        Body::Chiron => Some("Chiron_symbol_(fixed_width).svg"),
        // These aren't present in the legacy packs.
        _ => None,
    }
}
