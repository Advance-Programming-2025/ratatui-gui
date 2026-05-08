use ratatui::layout::Constraint;

pub fn explorers_columns() -> [Constraint; 4] {
    [
        Constraint::Length(3),
        Constraint::Length(7),
        Constraint::Fill(3),
        Constraint::Length(7),
    ]
}

pub fn planets_columns() -> [Constraint; 5] {
    [
        Constraint::Length(4),
        Constraint::Min(7),
        Constraint::Min(7),
        Constraint::Min(7),
        Constraint::Min(7),
    ]
}

