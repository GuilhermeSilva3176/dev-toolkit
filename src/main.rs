mod financas;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/financas/juros-simples", get(financas::juros_simples::calcular))
    .route("/financas/juros-compostos", get(financas::juros_compostos::calcular))
    .route("/financas/desconto-simples", get(financas::desconto_simples::calcular))
    .route("/financas/taxa-juros-implicita", get(financas::taxa_juros_implicita::calcular));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}