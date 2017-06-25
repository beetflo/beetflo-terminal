mod piano;
mod track_overview;

pub use self::piano::{Piano, Keys};
pub use self::track_overview::{TrackOverview};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        master,
        header,
        body,
        body_primary,
        body_secondary,
        body_left,
        body_right,
        footer,
        sidebar,
        content,

        heading_label,

        track_overview,
        piano,
    }
}
