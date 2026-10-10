use axum::{
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct CalcRequest {
    a: f64,
    b: f64,
    operation: String,
}

#[derive(Serialize)]
struct CalcResponse {
    result: Option<f64>,
    error: Option<String>,
}

async fn calculate(Json(data): Json<CalcRequest>) -> (StatusCode, Json<CalcResponse>) 
{
    let error = |message: &str| {
        (
            StatusCode::BAD_REQUEST,
            Json(CalcResponse {
                result: None,
                error: Some(message.to_string()),
            }),
        )
    };

    if !data.a.is_finite() || !data.b.is_finite() { return error("Некорректные числа"); }

    let result = match data.operation.as_str() {
        "+" => data.a + data.b,
        "-" => data.a - data.b,
        "*" => data.a * data.b,
        "/" => {
            if data.b == 0.0 {
                return error("Деление на ноль");
            }
            data.a / data.b
        }
        _ => return error("Неизвестная операция"),
    };

    if !result.is_finite() {
        return error("Результат вне допустимого диапазона");
    }

    (
        StatusCode::OK,
        Json(CalcResponse {
            result: Some(result),
            error: None,
        }),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/calculate", post(calculate))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Сервер запущен на порту 3000");

    axum::serve(listener, app).await.unwrap();
}