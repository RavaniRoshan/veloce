//! The 15-second demo: live frame timings, a layout-cache switch, and a
//! panic that restores the terminal. Same code as the installable binary.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    veloce_tui::showcase::run().await
}
