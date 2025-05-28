use axum::body::Body;
use axum::http::Request;
use backend::create_app; // Assuming lib.rs exposes create_app
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tower::ServiceExt;

fn health_check_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("health_check", |b| {
        b.to_async(&rt).iter(|| async {
            let app = create_app();
            let request = Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap();

            // Use black_box to prevent the compiler from optimizing away the call
            black_box(app.oneshot(request).await.unwrap())
        })
    });
}

criterion_group!(benches, health_check_benchmark);
criterion_main!(benches);
