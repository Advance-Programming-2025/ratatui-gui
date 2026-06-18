use std::collections::HashMap;

use common_game::components::resource::ResourceType;
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Cell, Paragraph, Row, Table},
};

use crate::ui::{
    layout,
    theme::{BlockThemeExt, SpanThemeExt, Theme},
};
use crate::view_models;
use crate::{app::App, trait_list::Printable};

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

    //render explorer selection
    frame.render_stateful_widget(table, area, app.ui.selectors.explorers.state_mut());

    //update planets where the selected explorer is
    let explorer_selected = app.ui.selectors.explorers.selected();
    match explorer_selected {
        None => {}
        Some(explorer) => {
            let explorer_planet = app.explorers_info.get_current_planet(&(explorer as u32));
            match explorer_planet {
                None => {}
                Some(planet) => app.ui.selectors.planets.set_last_selected(planet as usize),
            }
        }
    }
}

/// Render explorer detail panel.
pub(crate) fn render_extra_info_explorer(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let block = Block::bordered()
        .title(" Extra Info - Explorer ")
        .panel(theme);

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Recover the current explorer ID
    let explorer_id = app.get_id_selected_explorer();

    // Prepare the interface details
    let mut details = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  ID Explorer: ").default(theme),
            Span::styled(format!("{}", explorer_id), theme.value().bold()),
        ]),
        Line::from(vec![
            Span::raw("  Current Planet: ").default(theme),
            Span::styled(app.get_planet_selected_explorer(), theme.value()),
        ]),
        Line::from(""),
        Line::from(vec![Span::raw("  Bag Content:").default(theme)]),
    ];

    // Recover the list of the bag resources
    if let Some(explorer_data) = app
        .explorers_info
        .get(&(app.ui.selectors.explorers.last_selected().unwrap() as u32))
    {
        let mut bag_lines = get_formatted_bag_contents(&explorer_data.bag, theme);
        details.append(&mut bag_lines);
    } else {
        details.push(Line::from(vec![
            Span::raw("    ").default(theme),
            Span::styled("No explorer selected or data missing", theme.value().red()),
        ]));
    }

    // Draw the info
    let paragraph = Paragraph::new(details).style(theme.value());
    frame.render_widget(paragraph, inner_area);
}

/// Takes the list of explorer resources and returns a vector of Lines
/// sorted in a predefined order.
fn get_formatted_bag_contents<'a>(bag: &'a [ResourceType], theme: &'a Theme) -> Vec<Line<'a>> {
    if bag.is_empty() {
        return vec![Line::from(vec![
            Span::raw("    ").default(theme),
            Span::styled("Empty", theme.value().italic()),
        ])];
    }

    // Define the exact required order (based on the names returned by to_print())
    const RESOURCE_ORDER: &[&str] = &[
        "AI", "Robot", "Dolphin", "Life", "Water", "Diamond", "Silicon", "Oxygen", "Carbon",
        "Hydrogen",
    ];

    // Collect the resource counts from the bag into a HashMap
    let mut counts = HashMap::new();
    for resource in bag {
        let name = resource.to_print();
        *counts.entry(name).or_insert(0) += 1;
    }

    // Generate the text lines strictly following the array order
    let mut lines = Vec::new();
    for &resource_name in RESOURCE_ORDER {
        if let Some(&count) = counts.get(resource_name) {
            if count > 0 {
                lines.push(Line::from(vec![
                    Span::raw("    ").default(theme), // Rientranza elenco
                    Span::raw(format!("{}: ", resource_name)).default(theme),
                    Span::styled(format!("{}", count), theme.value().bold()),
                ]));
            }
        }
    }

    // Safety handling: if there are resources in the bag that were not included
    // in the RESOURCE_ORDER array, print them at the end so they aren't lost
    for (resource_name, count) in counts {
        if !RESOURCE_ORDER.contains(&resource_name.as_str()) && count > 0 {
            lines.push(Line::from(vec![
                Span::raw("    ").default(theme),
                Span::raw(format!("{}: ", resource_name)).default(theme),
                Span::styled(format!("{}", count), theme.value().bold()),
            ]));
        }
    }

    lines
}
