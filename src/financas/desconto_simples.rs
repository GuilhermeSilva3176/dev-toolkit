use axum::{Json, extract::Query, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CompararDescontoParams {
    pub valor_original: f64,
    pub taxa_desconto: f64, // % de desconto para pagamento à vista
    pub parcelas: u32,
    pub taxa_juros_parcelado: f64, // % de juros para pagamento parcelado
}

#[derive(Serialize)]
pub struct DescontoResponse {
    pub valor_a_vista: f64,
    pub valor_parcela: f64,
    pub valor_total_parcelado: f64,
    pub diferenca: f64,
}

pub async fn calcular(Query(params): Query<CompararDescontoParams>) -> Result<Json<DescontoResponse>, (StatusCode, String)> {
    
    if params.valor_original <= 0.0  {
        return Err((StatusCode::BAD_REQUEST, "Valor original deve ser maior que zero".to_string()));
    }

    if params.taxa_desconto < 0.0 || params.taxa_desconto > 100.0 {
        return Err((StatusCode::BAD_REQUEST, "Taxa de desconto deve estar entre 0 e 100".to_string()));
    }

    if params.parcelas == 0 {
        return Err((StatusCode::BAD_REQUEST, "Número de parcelas deve ser maior que zero".to_string()));
    }

    if params.taxa_juros_parcelado < 0.0 || params.taxa_juros_parcelado > 100.0 {
        return Err((StatusCode::BAD_REQUEST, "Taxa de juros para parcelamento deve estar entre 0 e 100".to_string()));
    }
    
    let valor_a_vista = params.valor_original - (params.valor_original * params.taxa_desconto / 100.0);
    let valor_total_parcelado = params.valor_original * (1.0 + params.taxa_juros_parcelado / 100.0 * (params.parcelas as f64));
    let valor_parcela = valor_total_parcelado / (params.parcelas as f64);
    let diferenca = valor_total_parcelado - valor_a_vista;

    Ok(Json(DescontoResponse {
        valor_a_vista,
        valor_parcela,
        valor_total_parcelado,
        diferenca,
    }))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn valor_original_negativo_retorna_erro() {
        // Arrange
        let params = CompararDescontoParams {
            valor_original: -1000.0,
            taxa_desconto: 10.0,
            parcelas: 12,
            taxa_juros_parcelado: 5.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn valor_original_zero_retorna_erro() {
        // Arrange
        let params = CompararDescontoParams {
            valor_original: 0.0,
            taxa_desconto: 10.0,
            parcelas: 12,
            taxa_juros_parcelado: 5.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_desconto_negativa_retorna_erro() {
        // Arrange
        let params = CompararDescontoParams {
            valor_original: 1000.0,
            taxa_desconto: -10.0,
            parcelas: 12,
            taxa_juros_parcelado: 5.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_desconto_maior_que_100_retorna_erro() {
        // Arrange
        let params = CompararDescontoParams {
            valor_original: 1000.0,
            taxa_desconto: 150.0,
            parcelas: 12,
            taxa_juros_parcelado: 5.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_juros_parcelado_negativa_retorna_erro() {
        // Arrange
        let params = CompararDescontoParams {
            valor_original: 1000.0,
            taxa_desconto: 10.0,
            parcelas: 12,
            taxa_juros_parcelado: -5.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_juros_parcelado_maior_que_100_retorna_erro() {
        // Arrange
        let params = CompararDescontoParams {
            valor_original: 1000.0,
            taxa_desconto: 10.0,
            parcelas: 12,
            taxa_juros_parcelado: 150.0,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn desconto_simples_correto() {
        // Arrange
        let params = CompararDescontoParams {
            valor_original: 1000.0,
            taxa_desconto: 10.0,
            parcelas: 12,
            taxa_juros_parcelado: 5.0,
        };

        // Act
        let resultado = calcular(Query(params)).await.unwrap().0;
        
        // Assert
        assert_eq!(resultado.valor_a_vista, 900.0);
        assert_eq!(resultado.valor_parcela, 133.33333333333334);
        assert_eq!(resultado.valor_total_parcelado, 1600.0);
        assert_eq!(resultado.diferenca, 700.0);
    }
}