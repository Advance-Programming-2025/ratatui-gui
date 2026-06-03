use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Cell, List, ListItem, Paragraph, Row, Table},
};

use crate::{
    app::App,
    ui::{
        layout,
        theme::{BlockThemeExt, SpanThemeExt, Theme},
    },
    ui_state::UiMode,
    view_models,
};

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
                UiMode::Normal | UiMode::GenerateResource { .. } => {
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
pub(crate) fn render_extra_info_planet(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
) {
    let outer = Block::bordered()
        .title(" Extra Info - Planet ")
        .panel(theme);
    let inner_area = outer.inner(area);
    frame.render_widget(outer, area);

    let [details_area, resources_area] =
        Layout::vertical([Constraint::Length(6), Constraint::Min(3)]).areas(inner_area);

    let text = vec![
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
    ];

    frame.render_widget(Paragraph::new(text), details_area);

    let resources = app.available_resources_for_selected_planet();
    let items: Vec<ListItem> = if resources.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  No supported resources",
            theme.value(),
        )]))]
    } else {
        resources
            .into_iter()
            .map(|resource| {
                ListItem::new(Line::from(vec![
                    Span::styled(resource, theme.value()),
                ]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(
            Block::bordered()
                .title(" Supported Resources ")
                .panel(theme),
        )
        .highlight_style(theme.row_highlight())
        .highlight_symbol("");

    if matches!(app.ui.mode, UiMode::GenerateResource { .. }) {
        frame.render_stateful_widget(list, resources_area, app.ui.selectors.resources.state_mut());
    } else {
        frame.render_widget(list, resources_area);
    }
}
