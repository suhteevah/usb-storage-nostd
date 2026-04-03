//! Block device trait for storage backends.
//!
//! This is a self-contained copy of the block device abstraction so
//! `usb-storage-nostd` can compile standalone without depending on `vfs-nostd`.

/// Trait for raw block storage backends.
///
/// Implementations must be safe to share across async tasks (`Send + Sync`).
pub trait BlockDevice: Send + Sync {
    /// Read `buf.len()` bytes starting at byte offset `offset` from the device.
    fn read_bytes(&self, offset: u64, buf: &mut [u8]) -> Result<usize, DeviceError>;

    /// Write `data` starting at byte offset `offset` to the device.
    fn write_bytes(&self, offset: u64, data: &[u8]) -> Result<usize, DeviceError>;

    /// Flush any cached writes to persistent storage.
    fn flush(&self) -> Result<(), DeviceError>;

    /// Return the sector size in bytes (typically 512 or 4096).
    fn sector_size(&self) -> u32;

    /// Return the total size of the device in bytes.
    fn total_size(&self) -> u64;
}

/// Errors from block device operations.
#[derive(Debug)]
pub enum DeviceError {
    /// A hardware or bus-level I/O error.
    IoError,
    /// The requested offset or length is beyond the device boundary.
    OutOfBounds,
    /// The device is not ready.
    NotReady,
    /// The operation timed out waiting for hardware.
    Timeout,
}

impl core::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IoError => write!(f, "I/O error"),
            Self::OutOfBounds => write!(f, "out of bounds"),
            Self::NotReady => write!(f, "not ready"),
            Self::Timeout => write!(f, "timeout"),
        }
    }
}
