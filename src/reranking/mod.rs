const DEFAULT_MAX_LENGTH: usize = 512;
#[cfg(not(target_family = "wasm"))]
const DEFAULT_BATCH_SIZE: usize = 256;

mod init;
pub use init::*;

mod r#impl;
