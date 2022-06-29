fn main() {
    tracing_subscriber::fmt()
    // enable everything
    .with_max_level(tracing::Level::TRACE)
    // sets this to be the default, global collector for this application.
    .init();

    tracing::trace!("Foobar");
}
