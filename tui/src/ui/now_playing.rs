use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

pub struct NowPlaying<'a> {
    pub app: &'a App,
}

impl<'a> Widget for NowPlaying<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(track) = self
            .app
            .states
            .blocking_lock()
            .queue_snapshot
            .current_track
            .as_ref()
        {
            let block = Block::bordered()
                .title("test")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded);

            let text = format!(
                "{}\n\
                            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                            Press left and right to increment and decrement the counter respectively.\n\
                        ",
                track.metadata.title
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .fg(Color::Cyan)
                .bg(Color::Black)
                .centered();

            paragraph.render(area, buf);
        }
    }
}
