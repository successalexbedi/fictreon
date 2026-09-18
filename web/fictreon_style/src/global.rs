use crate::tokens::fictreon_dark;

chain_ui_style::global! {
    * {
        box_sizing: border-box;
    }
    html {
        background: fictreon_dark.colors.surface;
    }
    body {
        margin: "0";
        font_family: "system-ui, sans-serif";
        background: fictreon_dark.colors.surface;
        color: fictreon_dark.colors.text;
    }
}