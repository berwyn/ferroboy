// ? Do these fields need to actually be exposed on the external interface?
// Might be better off having pub get and pub(crate) set
/// Configuration options for the emulation.
///
/// This should be used to configure how emulation is handled for a
/// given context, e.g. in the case of libretro doing a full bootcheck
/// is probably desireable, but a test harness might not care.
#[derive(Copy, Clone, Debug)]
pub struct Config {
    /// Whether or not to ensure that the ROM boots correctly like a
    /// real cartridge would, e.g. with the appropriate logo bitmap
    /// and initial JMP.
    pub enable_boot_check: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enable_boot_check: true,
        }
    }
}

pub struct ConfigBuilder {
    enable_boot_check: bool,
}

impl ConfigBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            enable_boot_check: true,
        }
    }

    pub fn without_boot_check(mut self) -> Self {
        self.enable_boot_check = false;
        self
    }

    pub fn build(&self) -> Config {
        Config {
            enable_boot_check: self.enable_boot_check,
        }
    }
}
