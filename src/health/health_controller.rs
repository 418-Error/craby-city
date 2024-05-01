use crate::health::health_service::HealthService;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Arc;
use tracing::info;

#[derive(Serialize)]
struct HealthCheckResponse {
    healthy: bool,
    report: HealthCheckReport,
}

#[derive(Serialize)]
struct HealthCheckReport {
    env: HealthCheckItem,
    postgres: HealthCheckDatabase,
}

#[derive(Serialize)]
struct HealthCheckItem {
    display_name: String,
    health: Health,
}

#[derive(Serialize)]
struct HealthCheckDatabase {
    display_name: String,
    health: Health,
    meta: Vec<DatabaseMeta>,
}

#[derive(Serialize)]
struct Health {
    healthy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Serialize)]
struct DatabaseMeta {
    connection: String,
    message: String,
    error: Option<String>,
}

#[get("/health/live")]
async fn live() -> impl Responder {
    info!("I'm alive!");
    HttpResponse::Ok().body("I'm alive!")
}

#[get("/health/readiness")]
async fn ready(health_service: web::Data<Arc<HealthService>>) -> impl Responder {
    info!("I'm alive and ready!");
    let db_health = match health_service.check().await {
        Ok(_) => Health {
            healthy: true,
            message: Some("All connections are healthy".to_string()),
        },
        Err(_) => Health {
            healthy: false,
            message: Some("Database connection failed".to_string()),
        },
    };

    let response = HealthCheckResponse {
        healthy: db_health.healthy,
        report: HealthCheckReport {
            env: HealthCheckItem {
                display_name: "Node Env Check".to_string(),
                health: Health {
                    healthy: true,
                    message: None,
                },
            },
            postgres: HealthCheckDatabase {
                display_name: "Database".to_string(),
                health: db_health,
                meta: vec![DatabaseMeta {
                    connection: "pg".to_string(),
                    message: "Connection is healthy".to_string(),
                    error: None,
                }],
            },
        },
    };

    HttpResponse::Ok().json(response)
}
