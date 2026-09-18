use axum::{response::Html, routing::get, Router};
use chain_ui_core::prelude::*;
use fictreon_style::{fictreon_theme, components::coming_soon::ComingSoonBadge, pages::home::Home}; 
use chain_ui_style::ext::StyleExt;

async fn home() -> Html<String> {
    let page = tag::html()
        .child(
            tag::head()
                .child(tag::title().child("Fictreon"))
                .child(fictreon_style::fictreon_theme()),
        )
        .child(
            tag::body().child(
                tag::div()
                    .style::<fictreon_style::pages::home::Home>()
                    .child(tag::h1().class("title").child("Fictreon"))
                    .child(
                        tag::span()
                            .style::<fictreon_style::components::coming_soon::ComingSoonBadge>()
                            .child("COMING SOON"),
                    )
                    .child(
                        tag::p()
                            .class("subtitle")
                            .child("Serialized fiction, built from the ground up. Real pages landing soon."),
                    ),
            ),
        );

    Html(page.build().into_string())
}

#[tokio::main]
async fn main() {
    let _ = fictreon_style::fictreon_css(); // forces theme resolution at startup

    let app = Router::new().route("/", get(home));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("fictreon running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}