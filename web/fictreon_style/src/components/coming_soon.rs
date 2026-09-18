use crate::tokens::fictreon_dark;

chain_ui_style::style!(coming_soon_badge {
    display: inline-block;
    padding: "6px 14px";
    border_radius: fictreon_dark.radius.md;
    background: fictreon_dark.colors.elevated;
    color: fictreon_dark.colors.gold;
    font_weight: "700";
    font_size: "13px";
    letter_spacing: "1px";
});