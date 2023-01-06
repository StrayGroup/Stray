#[cfg(feature="stray_render")]
pub mod render{
    pub use stray_render::*;
}

#[cfg(feature="stray_ecs")]
pub mod ecs{
    pub use stray_ecs::*;
}

#[cfg(feature="stray_material")]
pub mod material{
    pub use stray_material::*;
}

#[cfg(not(feature = "no-default-features"))]
pub mod prelude{
    pub use stray_render::*;
    pub use stray_ecs::*;
    pub use stray_material::*;
    pub use stray_api::*;
}

