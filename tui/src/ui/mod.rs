use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{ListState, StatefulWidget, TableState, Widget},
};

use crate::{
    app::App,
    ui::{library::Library, now_playing::NowPlaying, queue::Queue},
};

pub mod library;
pub mod now_playing;
pub mod queue;

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

        let lower_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(layout[1]);

        let queue = Queue { app: &self };
        queue.render(lower_layout[0], buf, &mut ListState::default());

        let library = Library { app: &self };
        library.render(lower_layout[1], buf, &mut TableState::default());
    }
}
