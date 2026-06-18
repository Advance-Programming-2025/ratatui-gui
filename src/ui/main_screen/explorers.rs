use std::collections::{HashMap};

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

    // 1. Recuperiamo l'ID corrente dell'explorer selezionato
    let explorer_id = app.get_id_selected_explorer();

    // 2. Prepariamo i dettagli base dell'interfaccia
    let mut details = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  ID Explorer: ").muted(theme),
            Span::styled(format!("{}", explorer_id), theme.value().bold()),
        ]),
        Line::from(vec![
            Span::raw("  Current Planet: ").muted(theme),
            Span::styled(app.get_planet_selected_explorer(), theme.value()),
        ]),
        Line::from(""),
        Line::from(vec![Span::raw("  Bag Content:").muted(theme)]),
    ];

    // 3. Recuperiamo il vettore reale di ResourceType dell'explorer selezionato dalla mappa del gioco
    if let Some(explorer_data) = app
        .explorers_info
        .get(&(app.ui.selectors.explorers.last_selected().unwrap() as u32))
    {
        // Otteniamo le linee formattate ("Hydrogen: 1", ecc.) e le appendiamo a details
        let mut bag_lines = get_formatted_bag_contents(&explorer_data.bag, theme);
        details.append(&mut bag_lines);
    } else {
        details.push(Line::from(vec![
            Span::raw("    ").muted(theme),
            Span::styled("No explorer selected or data missing", theme.value().red()),
        ]));
    }

    // 4. Disegniamo l'intero blocco di testo nel pannello
    let paragraph = Paragraph::new(details).style(theme.value());
    frame.render_widget(paragraph, inner_area);
}

/// Prende la lista di risorse dell'explorer e restituisce un vettore di Line
/// ordinate secondo un ordine specifico prefissato.
fn get_formatted_bag_contents<'a>(bag: &'a [ResourceType], theme: &'a Theme) -> Vec<Line<'a>> {
    if bag.is_empty() {
        return vec![Line::from(vec![
            Span::raw("    ").muted(theme),
            Span::styled("Empty", theme.value().italic()),
        ])];
    }

    // 1. Definiamo l'ordine esatto richiesto (basato sui nomi restituiti da to_print())
    // Nota: "AI" corrisponde a ComplexResourceType::AIPartner nel tuo trait_list.rs
    const RESOURCE_ORDER: &[&str] = &[
        "AI", "Robot", "Dolphin", "Life", "Water", "Diamond", "Silicon", "Oxygen", "Carbon",
        "Hydrogen",
    ];

    // 2. Raccogliamo i conteggi delle risorse presenti nella borsa dentro una HashMap
    let mut counts = HashMap::new();
    for resource in bag {
        let name = resource.to_print();
        *counts.entry(name).or_insert(0) += 1;
    }

    // 3. Generiamo le righe di testo seguendo RIGIDAMENTE l'ordine dell'array
    let mut lines = Vec::new();
    for &resource_name in RESOURCE_ORDER {
        // Se l'explorer ha almeno 1 unità di questa risorsa, creiamo la riga
        if let Some(&count) = counts.get(resource_name) {
            if count > 0 {
                lines.push(Line::from(vec![
                    Span::raw("    ").muted(theme), // Rientranza elenco
                    Span::raw(format!("{}: ", resource_name)).muted(theme),
                    Span::styled(format!("{}", count), theme.value().bold()),
                ]));
            }
        }
    }

    // Gestione di sicurezza: se ci sono risorse nella borsa che non erano incluse
    // nell'array RESOURCE_ORDER, le stampiamo in coda per non perderle
    for (resource_name, count) in counts {
        if !RESOURCE_ORDER.contains(&resource_name.as_str()) && count > 0 {
            lines.push(Line::from(vec![
                Span::raw("    ").muted(theme),
                Span::raw(format!("{}: ", resource_name)).muted(theme),
                Span::styled(format!("{}", count), theme.value().bold()),
            ]));
        }
    }

    lines
}
