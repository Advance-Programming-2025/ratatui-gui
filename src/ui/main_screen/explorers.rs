use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Cell, Row, Table},
};

use crate::app::App;
use crate::ui::{
    layout,
    theme::{BlockThemeExt, Theme},
};
use crate::view_models;
use super::*;

/// Render explorers table (list view).
pub(crate) fn render_explorers(app: &mut App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let header = Row::new(vec!["ID", "Status", "Bag", "Planet"]).style(theme.header());

    let rows: Vec<Row> = view_models::explorer_rows(app)
        .into_iter()
        .map(|row| {
            Row::new(vec![
                Cell::from(row.id.to_string()),
                Cell::from(row.status.to_string()),
                Cell::from(row.bag),
                Cell::from(row.planet_id),
            ])
        })
        .collect();

    let table = Table::new(rows, layout::explorers_columns())
        .header(header)
        .style(theme.value())
        .block(Block::bordered().title(" Explorers ").panel_active(theme))
        .row_highlight_style(theme.row_highlight());

    frame.render_stateful_widget(table, area, app.ui.selectors.explorers.state_mut());
}

/// Render explorer detail panel.
pub(crate) fn render_extra_info_explorer(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let block = Block::bordered()
        .title(" Extra Info - Explorer ")
        .panel(theme);

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let details = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  ID Explorer: ").muted(theme),
            Span::styled(
                format!("{}", app.get_id_selected_explorer()),
                theme.value().bold(),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Current Planet: ").muted(theme),
            Span::styled(app.get_planet_selected_explorer(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Bag: ").muted(theme),
            Span::styled(app.get_bag_selected_explorer(), theme.value()),
        ]),
    ];
    frame.render_widget(Paragraph::new(details), inner_area);
}

