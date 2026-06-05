use axum::{extract::Query, Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JurosParams {
    pub capital: f64,
    pub taxa: f64,
    pub tempo: u32,
}

#[derive(Serialize)]
pub struct JurosResponse {
    pub juros: f64,
    pub montante: f64,
}

pub async fn calcular(Query(params): Query<JurosParams>) -> Result<Json<JurosResponse>, (StatusCode, String)> {
    if params.capital <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Capital deve ser maior que zero".to_string()));
    }

    if params.taxa < 0.0 || params.taxa > 100.0 {
        return Err((StatusCode::BAD_REQUEST, "Taxa de juros deve estar entre 0 e 100".to_string()));
    }

    if params.tempo == 0 {
        return Err((StatusCode::BAD_REQUEST, "Tempo deve ser maior que zero".to_string()));
    }

    let juros = params.capital * params.taxa / 100.0 * (params.tempo as f64);
    let montante = params.capital + juros;

    Ok(Json(JurosResponse { juros, montante }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn capital_negativo_retorna_erro() {

        // Arrange

        let params = JurosParams{
            capital: -1000.0,
            taxa: 5.0,
            tempo: 12,
        };
        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn capital_zero_retorna_erro() {
        // Arrange
        let params = JurosParams{
            capital: 0.0,
            taxa: 5.0,
            tempo: 12,
        };
        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_negativa_retorna_erro() {
        // Arrange
        let params = JurosParams{
            capital: 1000.0,
            taxa: -5.0,
            tempo: 12,
        };
        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_maior_que_100_retorna_erro() {
        // Arrange
        let params = JurosParams{
            capital: 1000.0,
            taxa: 150.0,
            tempo: 12,
        };
        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn tempo_zero_retorna_erro() {
        // Arrange
        let params = JurosParams{
            capital: 1000.0,
            taxa: 5.0,
            tempo: 0,
        };
        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn calcular_juros_simples_retorna_resultado_correto() {
        // Arrange
        let params = JurosParams{
            capital: 1000.0,
            taxa: 5.0,
            tempo: 12,
        };
        // Act
        let resultado = calcular(Query(params)).await.unwrap().0;

        // Assert
        assert_eq!(resultado.juros, 600.0);
        assert_eq!(resultado.montante, 1600.0);
    }
}