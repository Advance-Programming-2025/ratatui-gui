use ratatui::{
    Frame, layout::Rect, text::{Line, Span}, widgets::{Block, Cell, List, Paragraph, Row, Table}
};

use crate::{app::App, ui::{layout, theme::{BlockThemeExt, SpanThemeExt}}, ui_state::UiMode, view_models};
use super::*;

/// Render planets table (list view).
/// Row styling uses view model flags (neighbor highlight) instead of inline logic.
pub(crate) fn render_planets_table(app: &mut App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let header =
        Row::new(vec!["ID", "Status", "Rocket", "Energy", "Incoming"]).style(theme.header());

    let rows: Vec<Row> = view_models::planet_rows(app)
        .into_iter()
        .map(|row| {
            let row_style = match app.ui.mode {
                UiMode::MoveExplorer { explorer_id, .. } => {
                    let explorer_planet = app
                        .explorers_info
                        .get(&explorer_id)
                        .map(|e| e.current_planet_id);
                    let allowed = match explorer_planet {
                        Some(pid) => {
                            row.id == pid || app.galaxy_topology[row.id as usize][pid as usize]
                        }
                        None => false,
                    };
                    if allowed {
                        theme.value()
                    } else {
                        theme.danger()
                    }
                }
                UiMode::Normal => {
                    if row.highlight_neighbor {
                        theme.value()
                    } else {
                        theme.danger()
                    }
                }
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

    frame.render_stateful_widget(table, area, app.ui.selectors.planets.state_mut());
}

/// Renders planet details when a planet is selected in the table.
pub(crate) fn render_extra_info_planet(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    //format the values for supported resource and supported combination
    let mut text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Name: ").muted(theme),
            Span::styled(app.get_name_selected_planet(), theme.value().bold()),
        ]),
        Line::from(vec![
            Span::raw("  ID: ").muted(theme),
            Span::styled(app.get_id_selected_planet(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Cells: ").muted(theme),
            Span::styled(app.get_cells_info_selected_planet(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Rocket: ").muted(theme),
            Span::styled(
                format!("{}", app.get_rocket_of_selected_planet()),
                theme.value(),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Supported Resource: ").muted(theme),
            Span::styled(
                format!("{}", app.get_supported_resource()),
                theme.value(),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Supported Combination: ").muted(theme),
            Span::styled(
                format!("{}", app.get_supported_combination()),
                theme.value(),
            ),
        ])
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Extra Info - Planet ")
            .panel(theme),
    );
    frame.render_widget(paragraph, area);
}
