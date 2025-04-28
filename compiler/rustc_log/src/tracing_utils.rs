#[cfg(all(feature = "tracing_chrome", feature = "tracing_tracy"))]
compile_error!(
    "Cannot enable both `tracing_chrome` and `tracing_tracy` features at the same time."
);

#[cfg(feature = "tracing_chrome")]
#[must_use]
pub struct TracingGuard {
    #[cfg(feature = "tracing_chrome")]
    _guard: tracing_chrome::FlushGuard,
}

#[cfg(not(feature = "tracing_chrome"))]
// #[must_use] TODO decide whether to add this or not
pub struct TracingGuard;

#[cfg(feature = "tracing_chrome")]
pub(crate) fn setup_tracing<S>() -> (TracingGuard, tracing_chrome::ChromeLayer<S>)
where
    S: tracing::Subscriber
        + for<'span> tracing_subscriber::registry::LookupSpan<'span>
        + Send
        + Sync,
{
    let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new().build();
    (TracingGuard { _guard: guard }, chrome_layer)
}

#[cfg(feature = "tracing_tracy")]
pub(crate) fn setup_tracing() -> (TracingGuard, tracing_tracy::TracyLayer) {
    (TracingGuard, tracing_tracy::TracyLayer::default())
}

#[cfg(not(any(feature = "tracing_chrome", feature = "tracing_tracy")))]
pub(crate) fn setup_tracing() -> (TracingGuard, tracing_subscriber::layer::Identity) {
    (TracingGuard, tracing_subscriber::layer::Identity::default())
}
