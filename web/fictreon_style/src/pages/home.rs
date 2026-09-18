use crate::tokens::fictreon_dark;

chain_ui_style::style!(home {
    display: flex;
    flex_direction: column;
    align_items: center;
    justify_content: center;
    height: "100vh";
    gap: fictreon_dark.spacing.md;
    text_align: center;

    .title {
        margin: "0";
        font_size: "32px";
        font_weight: "800";
    }

    .subtitle {
        margin: "0";
        color: fictreon_dark.colors.muted;
        max_width: "480px";
    }
});