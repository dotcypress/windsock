
/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

/// Metadata
pub mod metadata;

pub use self::instances::system;
pub use self::instances::gpioa;
pub use self::instances::uart1;
pub use self::instances::timer1;
