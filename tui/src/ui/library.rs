use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    text::ToText,
    widgets::{Cell, Row, StatefulWidget, Table, TableState},
};

use crate::state::DaemonStates;

pub struct Library<'a> {
    pub daemon_states: &'a DaemonStates,
}

impl<'a> StatefulWidget for Library<'a> {
    type State = TableState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let library = self.daemon_states.library_snapshot.clone();
        let header = ["Title".to_text().bold(), "Artist".to_text().bold()]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>();

        let entries: Vec<Row> = library
            .iter()
            .map(|(_, t)| {
                Row::new(vec![
                    t.metadata.title.clone(),
                    t.metadata.artist.clone().unwrap_or_default(),
                ])
            })
            .collect();

        let widths = [Constraint::Fill(1), Constraint::Percentage(30)];

        let highlight_style = Style::default().reversed();

        state.select(Some(self.daemon_states.library_selected_index));

        let table = Table::new(entries, widths)
            .header(header)
            .row_highlight_style(highlight_style);

        StatefulWidget::render(table, area, buf, state);
    }
}
