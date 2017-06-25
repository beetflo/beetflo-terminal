use conrod::{widget, Ui, Colorable, Positionable, Widget, Borderable, Sizeable};
// use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::position::Dimension;

use widgets;
use widgets::{Piano, TrackOverview};

use theme::{
    PRIMARY_SHADE,
    SECONDARY_SHADE,
    HEADER_BG,
    FOOTER_BG,
    TEXT_SHADE
};

pub struct BasicTopDownLayout {
    ids: widgets::Ids
}

impl BasicTopDownLayout {

    pub fn new(ui: &mut Ui) -> Self {
        BasicTopDownLayout {
            ids: widgets::Ids::new(ui.widget_id_generator())
        }
    }


    pub fn layout(&self, ui: &mut Ui) {
        let mut cell = ui.set_widgets();
        let w = cell.win_w;
        let h = cell.win_h;

        let header = widget::Canvas::new()
            .length(60.0)
            .color(HEADER_BG)
            .pad_left(20.0)
            .pad_right(20.0)
            .pad_top(15.0)
            .border_color(PRIMARY_SHADE);


        let body_primary = widget::Canvas::new()
            .border_color(SECONDARY_SHADE)
            .color(SECONDARY_SHADE);

        let body_secondary = widget::Canvas::new()
            .border_color(SECONDARY_SHADE)
            .color(SECONDARY_SHADE);

        let body_flow = &[
            (self.ids.body_primary, body_primary),
            (self.ids.body_secondary, body_secondary)
        ];

        let body   = widget::Canvas::new()
            .color(SECONDARY_SHADE)
            .pad(0.0)
            .crop_kids()
            .border_color(SECONDARY_SHADE)
            .flow_down(body_flow);

        let footer = widget::Canvas::new()
            .color(FOOTER_BG)
            .pad(0.0).length(40.0)
            .border_color(FOOTER_BG);

        widget::Canvas::new().flow_down(&[
            (self.ids.header, header),
            (self.ids.body,   body),
            (self.ids.footer, footer),
        ]).set(self.ids.master, &mut cell);


        widget::Text::new("Beetflo").color(TEXT_SHADE)
            .top_left_of(self.ids.header).font_size(21)
            .set(self.ids.heading_label, &mut cell);

        TrackOverview::new()
            .y_dimension(Dimension::Absolute(h * 0.5))
            .x_dimension(Dimension::Absolute(w))
            .top_left_with_margin_on(self.ids.body_primary, 0.0)
            .set(self.ids.track_overview, &mut cell);

        Piano::new()
            .y_dimension(Dimension::Absolute(h * 0.5))
            .x_dimension(Dimension::Absolute(w))
            .top_left_with_margin_on(self.ids.body_secondary, 0.0)
            .set(self.ids.piano, &mut cell);

    }
}
