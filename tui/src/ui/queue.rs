use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, StatefulWidget},
};

use crate::state::DaemonStates;

pub struct Queue<'a> {
    pub daemon_states: &'a DaemonStates,
}

impl<'a> StatefulWidget for Queue<'a> {
    type State = ListState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let entries: Vec<ListItem> = self
            .daemon_states
            .queue_snapshot
            .next_tracks
            .iter()
            .map(|t| ListItem::from(t.metadata.title.clone()))
            .collect();

        let highlight = Style::default().reversed();
        let list = List::new(entries)
            .highlight_style(highlight)
            .block(Block::new().title("Queue").bold())
            .not_bold();

        StatefulWidget::render(list, area, buf, state);
    }
}
