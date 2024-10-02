use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html("<h1>Open Seal Story Painter</h1>")
}

pub async fn health() -> &'static str {
    "200 OK"
}
