use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Cell, Row, Table},
};

use crate::app::App;
use crate::ui::{layout, theme::Theme};
use crate::view_models;

/// Render planets table (list view).
/// Row styling uses view model flags (neighbor highlight) instead of inline logic.
pub fn render_planets_table(app: &mut App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let header =
        Row::new(vec!["ID", "Status", "Rocket", "Energy", "Incoming"]).style(theme.header());

    let rows: Vec<Row> = view_models::planet_rows(app)
        .into_iter()
        .map(|row| {
            let row_style = if row.highlight_neighbor {
                theme.danger()
            } else {
                theme.value()
            };
            Row::new(vec![
                Cell::from(row.id.to_string()),
                Cell::from(row.status),
                Cell::from(row.rocket),
                Cell::from(row.energy),
                Cell::from(row.incoming),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(rows, layout::planets_columns())
        .header(header)
        .style(theme.value())
        .block(
            Block::bordered()
                .title(" Planets ")
                .border_style(theme.success()),
        )
        .row_highlight_style(theme.row_highlight());

    frame.render_stateful_widget(table, area, &mut app.planet_selector);
}
