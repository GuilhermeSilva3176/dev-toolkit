use axum::{extract::Query, Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum Periodicidade {
    Mensal,
    Anual,
    Semestral,
    Trimestral,
}

#[derive(Deserialize)]
pub struct JurosParams {
    pub capital: f64,
    pub taxa: f64,
    pub tempo: u32,
    pub aporte: Option<f64>,
    pub periodicidade: Option<Periodicidade>,
}

#[derive(Serialize)]
pub struct JurosResponse {
    pub valor_inicial: f64,
    pub total_aportado: f64,
    pub juros_total: f64,
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
    if let Some(aporte) = params.aporte {
        if aporte < 0.0 {
            return Err((StatusCode::BAD_REQUEST, "Aporte deve ser maior ou igual a zero".to_string()));
        }
    }

    let periodos_por_ano = match params.periodicidade {
        Some(Periodicidade::Mensal) => 12.0,
        Some(Periodicidade::Anual) => 1.0,
        Some(Periodicidade::Semestral) => 2.0,
        Some(Periodicidade::Trimestral) => 4.0,
        None => 12.0, // Assume padrão mensal
    };

    let taxa_ajustada = params.taxa / periodos_por_ano;
    let valor_inicial: f64 = params.capital;

    if let Some(aporte) = params.aporte {
        let mut montante: f64 = params.capital;
        for _ in 0..(params.tempo as usize) {
            montante *= 1.0 + taxa_ajustada / 100.0;
            montante += aporte;
        }
        let total_aportado: f64 = aporte * (params.tempo as f64);
        let juros_total: f64 = montante - params.capital - total_aportado;
        return Ok(Json(JurosResponse { valor_inicial, total_aportado, juros_total, montante }));
    }

    let montante: f64 = params.capital * (1.0 + taxa_ajustada / 100.0).powf(params.tempo as f64);
    let juros_total: f64 = montante - params.capital;

    Ok(Json(JurosResponse { valor_inicial, total_aportado: 0.0, juros_total, montante }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn capital_negativo_retorna_erro() {
        // Arrange
        let params = JurosParams {
            capital: -1000.0,
            taxa: 5.0,
            tempo: 12,
            aporte: None,
            periodicidade: None,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn capital_zero_retorna_erro() {
        // Arrange
        let params = JurosParams {
            capital: 0.0,
            taxa: 5.0,
            tempo: 12,
            aporte: None,
            periodicidade: None,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_negativa_retorna_erro() {
        // Arrange
        let params = JurosParams {
            capital: 1000.0,
            taxa: -5.0,
            tempo: 12,
            aporte: None,
            periodicidade: None,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn taxa_maior_que_100_retorna_erro() {
        // Arrange
        let params = JurosParams {
            capital: 1000.0,
            taxa: 150.0,
            tempo: 12,
            aporte: None,
            periodicidade: None,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn tempo_zero_retorna_erro() {
        // Arrange
        let params = JurosParams {
            capital: 1000.0,
            taxa: 5.0,
            tempo: 0,
            aporte: None,
            periodicidade: None,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn aporte_negativo_retorna_erro() {
        // Arrange
        let params = JurosParams {
            capital: 1000.0,
            taxa: 5.0,
            tempo: 12,
            aporte: Some(-100.0),
            periodicidade: None,
        };

        // Act
        let resultado = calcular(Query(params)).await;

        // Assert
        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn calculo_juros_simples_correto() {
        // Arrange
        let params = JurosParams {
            capital: 1000.0,
            taxa: 5.0,
            tempo: 12,
            aporte: None,
            periodicidade: None,
        };

        // Act
        let resultado = calcular(Query(params)).await.unwrap().0;

        // Assert
        assert_eq!(resultado.valor_inicial, 1000.0);
        assert_eq!(resultado.total_aportado, 0.0);
        assert_eq!(resultado.juros_total, 51.16189788173301);
        assert_eq!(resultado.montante, 1051.161897881733);
    }

    #[tokio::test]
    async fn calculo_juros_compostos_com_aporte_correto() {
        // Arrange
        let params = JurosParams {
            capital: 1000.0,
            taxa: 5.0,
            tempo: 12,
            aporte: Some(100.0),
            periodicidade: Some(Periodicidade::Mensal),
        };

        // Act
        let resultado = calcular(Query(params)).await.unwrap().0;

        // Assert
        assert_eq!(resultado.valor_inicial, 1000.0);
        assert_eq!(resultado.total_aportado, 1200.0);
        assert_eq!(resultado.juros_total, 79.04744704332916);
        assert_eq!(resultado.montante, 2279.047447043329);
    }
}