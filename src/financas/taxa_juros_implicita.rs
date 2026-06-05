use axum::{extract::Query, Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TaxaImplicitaParams {
    pub valor_original: f64,
    pub parcelas: u32,
    pub valor_parcela: f64,
}

#[derive(Serialize)]
pub struct TaxaImplicitaResponse {
    pub taxa_juros_implicita: f64,
}

pub async fn calcular(Query(params): Query<TaxaImplicitaParams>) -> Result<Json<TaxaImplicitaResponse>, (StatusCode, String)> {
    if params.valor_original <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Valor original deve ser maior que zero".to_string()));
    }

    if params.parcelas == 0 {
        return Err((StatusCode::BAD_REQUEST, "Número de parcelas deve ser maior que zero".to_string()));
    }

    if params.valor_parcela <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Valor da parcela deve ser maior que zero".to_string()));
    }

    let valor_total_parcelado = (params.parcelas as f64) * params.valor_parcela;
    let taxa_juros_implicita = ((valor_total_parcelado / params.valor_original).powf(1.0 / params.parcelas as f64) - 1.0) * 100.0;

    Ok(Json(TaxaImplicitaResponse { taxa_juros_implicita }))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn valor_original_negativo_retorna_erro() {
        // Arrange
        let params = TaxaImplicitaParams {
            valor_original: -1000.0,
            parcelas: 12,
            valor_parcela: 100.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn valor_original_zero_retorna_erro() {
        // Arrange
        let params = TaxaImplicitaParams {
            valor_original: 0.0,
            parcelas: 12,
            valor_parcela: 100.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn parcelas_zero_retorna_erro() {
        // Arrange
        let params = TaxaImplicitaParams {
            valor_original: 1000.0,
            parcelas: 0,
            valor_parcela: 100.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn valor_parcela_negativo_retorna_erro() {
        // Arrange
        let params = TaxaImplicitaParams {
            valor_original: 1000.0,
            parcelas: 12,
            valor_parcela: -100.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn valor_parcela_zero_retorna_erro() {
        // Arrange
        let params = TaxaImplicitaParams {
            valor_original: 1000.0,
            parcelas: 12,
            valor_parcela: 0.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn calculo_taxa_juros_implicita_correto() {
        // Arrange
        let params = TaxaImplicitaParams {
            valor_original: 1000.0,
            parcelas: 12,
            valor_parcela: 100.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_ok());
        let resposta = resultado.unwrap().0;
        assert!((resposta.taxa_juros_implicita - 1.53).abs() < 0.01); // Verifica se a taxa implícita está próxima de 1.53%
    }
}