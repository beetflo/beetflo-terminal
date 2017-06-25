mod piano;
mod track_overview;

pub use self::piano::{Piano};
pub use self::track_overview::{TrackOverview};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        master,
        header,
        body,
        footer,
        sidebar,
        content,

        heading_label,

        track_overview,
        piano,
    }
}
