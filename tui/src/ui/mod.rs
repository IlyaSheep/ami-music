use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, TableState, Widget},
};

use crate::{
    app::App,
    ui::{library::Library, now_playing::NowPlaying},
};

pub mod library;
pub mod now_playing;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let now_playing = NowPlaying { app: &self };
        now_playing.render(layout[0], buf);

        let library = Library { app: &self };
        library.render(layout[1], buf, &mut TableState::new());
    }
}
