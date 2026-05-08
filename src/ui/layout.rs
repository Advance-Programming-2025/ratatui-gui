use ratatui::layout::Constraint;

/// Column constraints for explorers table.
/// Keep widths centralized to avoid scattered magic numbers.
pub fn explorers_columns() -> [Constraint; 4] {
    [
        Constraint::Length(3),
        Constraint::Length(7),
        Constraint::Fill(3),
        Constraint::Length(7),
    ]
}

/// Column constraints for planets table.
/// Keep widths centralized to make future renames and localization safer.
pub fn planets_columns() -> [Constraint; 5] {
    [
        Constraint::Length(4),
        Constraint::Min(7),
        Constraint::Min(7),
        Constraint::Min(7),
        Constraint::Min(7),
    ]
}
