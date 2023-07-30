pub mod bindings;

pub mod api;
pub mod app;
mod archive;
mod error;
mod event_handler;
mod options;
pub mod utf;
mod value;
mod window;
mod debug;

pub use archive::*;
pub use error::*;
pub use event_handler::*;
pub use options::*;
pub use value::*;
pub use window::*;
pub use debug::*;

pub type Result<T> = std::result::Result<T, Error>;

// reexport macros
pub use rsciter_macro::xmod;

#[cfg(test)]
pub mod tests {
    use crate::api::{sapi, VersionKind};

    #[test]
    fn test() {
        let api = sapi().unwrap();
        let v0 = api.sciter_version(VersionKind::MAJOR).unwrap();
        let v1 = api.sciter_version(VersionKind::MINOR).unwrap();
        let v2 = api.sciter_version(VersionKind::UPDATE).unwrap();
        let v3 = api.sciter_version(VersionKind::BUILD).unwrap();
        let v4 = api.sciter_version(VersionKind::REVISION).unwrap();

        dbg!(api.graphics_caps().unwrap());

        println!(
            "\x1b[94mVersion:\x1b[0m {}, {}, {}, {}, {}, {}",
            api.version(),
            v0,
            v1,
            v2,
            v3,
            v4
        );
    }
}
