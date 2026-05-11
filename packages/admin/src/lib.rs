//! Admin console — separate binary that re-uses `ui::components`,
//! `ui::theme`, and the kit's `admin.css`. Backed by the GraphQL admin
//! operations re-exported through `ui::api`.

pub mod components;
pub mod state;
pub mod views;

pub use state::{AdminSection, CURRENT_JOB, CURRENT_SECTION, CURRENT_STAGE};
