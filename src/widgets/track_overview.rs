use conrod::{self, widget, Colorable, Dimensions, Labelable, Point, Positionable, Widget, UiCell};


pub struct TrackOverview {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    common: widget::CommonBuilder,

    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool
}


// We use the `widget_style!` macro to vastly simplify the definition and implementation of the
// widget's associated `Style` type. This generates both a `Style` struct, as well as an
// implementation that automatically retrieves defaults from the provided theme.
//
// See the documenation of the macro for a more details.
widget_style!{
    /// Represents the unique styling for our CircularButton widget.
    style Style {
        /// Color of the button.
        - color: conrod::Color { theme.shape_color }
        /// Color of the button's label.
        - label_color: conrod::Color { theme.label_color }
        /// Font size of the button's label.
        - label_font_size: conrod::FontSize { theme.font_size_medium }
        /// Specify a unique font for the label.
        - label_font_id: Option<conrod::text::font::Id> { theme.font_id }
    }
}

// We'll create the widget using a `Circle` widget and a `Text` widget for its label.
//
// Here is where we generate the type that will produce these identifiers.
widget_ids! {
    pub struct Ids {
        circle,
        text,
    }
}

/// Represents the unique, cached state for our CircularButton widget.
pub struct State {
    ids: Ids,
}

/// Return whether or not a given point is over a circle at a given point on a
/// Cartesian plane. We use this to determine whether the mouse is over the button.
pub fn is_over_circ(circ_center: Point, mouse_point: Point, dim: Dimensions) -> bool {
    // Offset vector from the center of the circle to the mouse.
    let offset = conrod::utils::vec2_sub(mouse_point, circ_center);

    // If the length of the offset vector is less than or equal to the circle's
    // radius, then the mouse is inside the circle. We assume that dim is a square
    // bounding box around the circle, thus 2 * radius == dim[0] == dim[1].
    let distance = (offset[0].powf(2.0) + offset[1].powf(2.0)).sqrt();
    let radius = dim[0] / 2.0;
    distance <= radius
}

impl TrackOverview {

    pub fn attach(ui: &mut UiCell) {

    }

    /// Create a button context to be built upon.
    pub fn new() -> Self {
        TrackOverview {
            common: widget::CommonBuilder::new(),
            style: Style::new(),
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// If true, will allow user inputs.  If false, will disallow user inputs.  Like
    /// other Conrod configs, this returns self for chainability. Allow dead code
    /// because we never call this in the example.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }
}



impl Widget for TrackOverview {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<()>;

    fn common(&self) -> &widget::CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut widget::CommonBuilder {
        &mut self.common
    }

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

        /// Update the state of the button by handling any input that has occurred since the last
        /// update.
        fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
            let widget::UpdateArgs { id, state, rect, mut ui, style, .. } = args;

            let (color, event) = {
                let input = ui.widget_input(id);

                // If the button was clicked, produce `Some` event.
                let event = input.clicks().left().next().map(|_| ());

                let color = style.color(&ui.theme);
                let color = input.mouse().map_or(color, |mouse| {
                    if is_over_circ([0.0, 0.0], mouse.rel_xy(), rect.dim()) {
                        if mouse.buttons.left().is_down() {
                            color.clicked()
                        } else {
                            color.highlighted()
                        }
                    } else {
                        color
                    }
                });

                (color, event)
            };

            // Finally, we'll describe how we want our widget drawn by simply instantiating the
            // necessary primitive graphics widgets.
            //
            // Conrod will automatically determine whether or not any changes have occurred and
            // whether or not any widgets need to be re-drawn.
            //
            // The primitive graphics widgets are special in that their unique state is used within
            // conrod's backend to do the actual drawing. This allows us to build up more complex
            // widgets by using these simple primitives with our familiar layout, coloring, etc
            // methods.
            //
            // If you notice that conrod is missing some sort of primitive graphics that you
            // require, please file an issue or open a PR so we can add it! :)

            // First, we'll draw the **Circle** with a radius that is half our given width.
            let radius = rect.w() / 2.0;
            widget::Circle::fill(radius)
                .middle_of(id)
                .graphics_for(id)
                .color(color)
                .set(state.ids.circle, ui);

            // Now we'll instantiate our label using the **Text** widget.
            let label_color = style.label_color(&ui.theme);
            let font_size = style.label_font_size(&ui.theme);
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            widget::Text::new("TrackOverview")
                .and_then(font_id, widget::Text::font_id)
                .middle_of(id)
                .font_size(font_size)
                .graphics_for(id)
                .color(label_color)
                .set(state.ids.text, ui);

            event
        }
}
