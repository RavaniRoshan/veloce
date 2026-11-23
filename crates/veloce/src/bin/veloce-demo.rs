//! Installable one-command demo for `veloce-tui`.
//!
//! ```bash
//! cargo install veloce-tui --bin veloce-demo
//! veloce-demo
//! ```

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    veloce_tui::showcase::run().await
}
