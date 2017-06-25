use conrod::{self, widget, Ui, Colorable, Positionable, Widget, UiCell, color, Borderable};
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use widgets;
use widgets::{Piano, TrackOverview};

use theme::{
    PRIMARY_SHADE,
    SECONDARY_SHADE,
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

        let header = widget::Canvas::new()
            .length(60.0)
            .color(PRIMARY_SHADE)
            .pad(10.0)
            .border_color(PRIMARY_SHADE);

        let body   = widget::Canvas::new()
            .color(SECONDARY_SHADE)
            .pad(10.0)
            .border_color(PRIMARY_SHADE);

        let footer = widget::Canvas::new()
            .color(FOOTER_BG)
            .pad(10.0).length(40.0)
            .border_color(PRIMARY_SHADE);

        widget::Canvas::new().flow_down(&[
            (self.ids.header, header),
            (self.ids.body,   body),
            (self.ids.footer, footer),
        ]).set(self.ids.master, &mut cell);


        widget::Text::new("Beetflo").color(TEXT_SHADE)
            .top_left_of(self.ids.header).font_size(26)
            .set(self.ids.heading_label, &mut cell);
    }
}
