//! Optional OpenTelemetry export (catalog E8) — enable bootstrap `otel` feature.

#[cfg(feature = "otel")]
pub fn init_otel_tracing(
    service_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::trace::TracerProvider;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:4318".to_owned());
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .build()?;
    let provider = TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .build();
    let tracer = provider.tracer(service_name.to_owned());
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer().fmt_fields(crate::tracing_init::RedactingFormatFields),
        )
        .with(telemetry)
        .try_init()?;
    Ok(())
}

#[cfg(not(feature = "otel"))]
pub fn init_otel_tracing(
    _service_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("bootstrap compiled without `otel` feature".into())
}
