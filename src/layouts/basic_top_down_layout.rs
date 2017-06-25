use conrod::{self, widget, Ui, Colorable, Positionable, Widget, UiCell, color};
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use widgets;
use widgets::{Piano, TrackOverview};

use theme::{
    PRIMARY_SHADE,
    SECONDARY_SHADE,
    FOOTER_BG
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

        widget::Canvas::new().flow_down(&[
            (self.ids.header, widget::Canvas::new().color(PRIMARY_SHADE).pad(10.0)),
            (self.ids.body, widget::Canvas::new().color(SECONDARY_SHADE).pad(10.0)),
            (self.ids.footer, widget::Canvas::new().color(FOOTER_BG)),
        ]).set(self.ids.master, &mut cell);


        widget::Text::new("Beetflo").color()
            .top_left_of(self.ids.header)
            .set(self.ids.heading_label, &mut cell);
    }
}
