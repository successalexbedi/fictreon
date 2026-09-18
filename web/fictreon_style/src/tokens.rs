chain_ui_style::contract!(ThemeTokens {
    colors { surface elevated text muted gold }
    spacing { xs sm md lg xl }
    radius { sm md lg }
});

chain_ui_style::tokens! {
    fictreon_dark: ThemeTokens {
        colors {
            surface: "#1E1E1E",
            elevated: "#252525",
            text: "#F5F3ED",
            muted: "#A8A8A8",
            gold: "#C8A45A",
        }
        spacing { xs: "4px", sm: "8px", md: "16px", lg: "24px", xl: "40px" }
        radius { sm: "6px", md: "10px", lg: "16px" }
    }
}