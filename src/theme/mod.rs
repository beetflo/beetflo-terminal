use conrod::{Color};

macro_rules! make_color {
	  ($r:expr, $g:expr, $b:expr) => ( Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0));
	  ($r:expr, $g:expr, $b:expr, $a:expr) => ( Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, $a as f32 / 255.0));
}

pub const PRIMARY_SHADE       : Color = make_color!(196.0,196.0,196.0);
pub const SECONDARY_SHADE     : Color = make_color!(196.0,196.0,196.0);

pub const FOOTER_BG           : Color = make_color!(196.0,196.0,196.0);

