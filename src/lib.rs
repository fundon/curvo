#![allow(clippy::needless_range_loop)]

mod adaptive_tessellation_node;
mod adaptive_tessellation_processor;
mod binomial;
mod floating_point;
mod frenet_frame;
mod invertible;
mod knot_vector;
mod nurbs_curve;
mod nurbs_surface;
mod surface_point;
mod surface_tessellation;
mod transformable;
mod trigonometry;
use floating_point::*;
use surface_point::*;

pub mod prelude {
    pub use crate::adaptive_tessellation_processor::AdaptiveTessellationOptions;
    pub use crate::floating_point::*;
    pub use crate::frenet_frame::*;
    pub use crate::invertible::*;
    pub use crate::knot_vector::*;
    pub use crate::nurbs_curve::*;
    pub use crate::nurbs_surface::*;
    pub use crate::surface_tessellation::*;
    pub use crate::transformable::*;
}
