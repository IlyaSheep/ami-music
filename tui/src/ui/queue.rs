use ratatui::{
    style::Style,
    widgets::{List, ListItem, ListState, StatefulWidget},
};

use crate::app::App;

pub struct Queue<'a> {
    pub app: &'a App,
}

impl<'a> StatefulWidget for Queue<'a> {
    type State = ListState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if let Ok(states) = self.app.states.try_lock() {
            let entries: Vec<ListItem> = states
                .queue_snapshot
                .next_tracks
                .iter()
                .map(|t| ListItem::from(t.metadata.title.clone()))
                .collect();

            let highlight = Style::default().reversed();
            let list = List::new(entries).highlight_style(highlight);

            StatefulWidget::render(list, area, buf, state);
        }
    }
}
