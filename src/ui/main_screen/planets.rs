use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Cell, List, ListItem, Paragraph, Row, Table},
};

use crate::{
    app::App,
    trait_list::Printable,
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

    let [basic_area, complex_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(resources_area);

    // Define the details extra info
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Name: ").default(theme),
            Span::styled(app.get_name_selected_planet(), theme.value().bold()),
        ]),
        Line::from(vec![
            Span::raw("  ID: ").default(theme),
            Span::styled(app.get_id_selected_planet(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Cells: ").default(theme),
            Span::styled(app.get_cells_info_selected_planet(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Rocket: ").default(theme),
            Span::styled(
                format!("{}", app.get_rocket_of_selected_planet()),
                theme.value(),
            ),
        ]),
    ];

    frame.render_widget(Paragraph::new(text), details_area);

    // Define the basic list widget
    let planet_id = app.get_selected_planet().unwrap() as u32;
    let basic_resources = app.get_supported_resource(planet_id);
    let items: Vec<ListItem> = if basic_resources.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  No supported resources",
            theme.value(),
        )]))]
    } else {
        basic_resources
            .clone()
            .into_iter()
            .map(|basic| {
                ListItem::new(Line::from(vec![Span::styled(
                    basic.to_print(),
                    theme.value(),
                )]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Basic Resources ").panel(theme))
        .highlight_style(theme.row_highlight())
        .highlight_symbol("");

    frame.render_stateful_widget(
        list,
        basic_area,
        app.ui.selectors.basic_resources.state_mut(),
    );

    // Define complex resource widget
    let planet_id = app.get_selected_planet().unwrap() as u32;
    let complex_resources = app.get_supported_combination(planet_id);
    let items: Vec<ListItem> = if complex_resources.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  None",
            theme.value(),
        )]))]
    } else {
        complex_resources
            .clone()
            .into_iter()
            .map(|complex| {
                ListItem::new(Line::from(vec![Span::styled(
                    complex.to_print(),
                    theme.value(),
                )]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Complex Resources ").panel(theme))
        .highlight_style(theme.row_highlight())
        .highlight_symbol("");

    frame.render_stateful_widget(
        list,
        complex_area,
        app.ui.selectors.complex_resource.state_mut(),
    );
}
