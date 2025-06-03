// This module provides utilities to set up OpenTelemetry tracing using the OTLP exporter.
// It configures the tracer provider, resource attributes, and integrates with tracing-subscriber.
use opentelemetry::trace::TracerProvider;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use std::{error::Error, sync::OnceLock};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

// get_resource initializes a global Resource containing service name and version.
// Uses OnceLock to ensure the resource is created only once.
fn get_resource() -> Resource {
    static RESOURCE: OnceLock<Resource> = OnceLock::new();
    RESOURCE
        .get_or_init(|| {
            // Build the Resource with service name and version attributes.
            Resource::builder()
                .with_service_name(env!("CARGO_PKG_NAME"))
                .with_attributes(vec![KeyValue::new(
                    "service.version",
                    env!("CARGO_PKG_VERSION"),
                )])
                .build()
        })
        .clone()
}

// init_traces sets up the OTLP exporter and builds the SdkTracerProvider.
pub fn init_traces() -> SdkTracerProvider {
    // Load environment variables from .env file.
    dotenvy::dotenv().ok();

    // Read the OTLP endpoint from environment variable.
    let otlp_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .expect("OTEL_EXPORTER_OTLP_ENDPOINT must be set");
    // Read the OTLP protocol (default to http/protobuf) from environment variable.
    let protocol_str = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL")
        .unwrap_or_else(|_| "http/protobuf".to_string());
    // Determine the OTLP protocol enum based on the protocol string.
    let protocol = match protocol_str.as_str() {
        "http/json" => Protocol::HttpJson,
        "http/protobuf" => Protocol::HttpBinary,
        other => panic!("Unsupported OTLP protocol: {}", other),
    };
    // Create the OTLP HTTP exporter using the specified endpoint and protocol.
    let exporter = match protocol {
        Protocol::HttpJson | Protocol::HttpBinary => {
            opentelemetry_otlp::HttpExporterBuilder::default()
                .with_endpoint(otlp_endpoint)
                .with_protocol(protocol)
                .build_span_exporter()
                .expect("Failed to create trace exporter")
        }
        _ => panic!("Unsupported OTLP protocol"),
    };

    // Build and return the tracer provider with batch exporter and resource.
    SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .build()
}

// setup_tracing_opentelemetry initializes tracing-subscriber with OpenTelemetry integration.
pub fn setup_tracing_opentelemetry() -> SdkTracerProvider {
    // Load environment variables from .env file.
    dotenvy::dotenv().ok();

    // Configure log level filter from environment or default to info for application and debug for opentelemetry.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,opentelemetry=debug".parse().unwrap());

    // Configure formatting layer for tracing-subscriber, including timestamp, thread info, and span events.
    let fmt_layer = tracing_subscriber::fmt::layer()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_target(false)
                .with_level(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339()),
        )
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(filter);

    // Initialize the OTLP tracer provider.
    let tracer_provider = init_traces();

    // Set the global tracer provider for OpenTelemetry.
    global::set_tracer_provider(tracer_provider.clone());

    // Obtain a tracer instance for this service.
    let tracer = tracer_provider.tracer(env!("CARGO_PKG_NAME"));
    // Create an OpenTelemetry layer for tracing-subscriber.
    let otel_layer = OpenTelemetryLayer::new(tracer);

    // Register the formatting layer and OpenTelemetry layer, then initialize the subscriber.
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(otel_layer)
        .init();

    tracer_provider
}

// shutdown_opentelemetry gracefully shuts down the tracer provider, ensuring all spans are exported.
pub fn shutdown_opentelemetry(
    tracer_provider: SdkTracerProvider,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Attempt to shut down the tracer provider and return any errors.
    if let Err(e) = tracer_provider.shutdown() {
        return Err(format!("tracer provider: {}", e).into());
    }
    Ok(())
}
