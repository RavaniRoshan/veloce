pub use veloce_core as core;
pub use veloce_layout as layout;
pub use veloce_render as render;
pub use veloce_router as router;

pub mod app;
pub use app::VeloceApp;
pub use veloce_runtime as runtime;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::VeloceApp;
    pub use veloce_core::*;
    pub use veloce_layout::*;
    pub use veloce_render::*;
    pub use veloce_router::*;
    pub use veloce_runtime::*;
}
