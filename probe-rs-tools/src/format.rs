use clap::ValueEnum;
use postcard_schema::Schema;
use probe_rs::Target;
use serde::{Deserialize, Serialize};

use crate::util::{parse_u32, parse_u64};

#[derive(clap::Parser, Clone, Serialize, Deserialize, Debug, Default, Schema)]
#[serde(default)]
pub struct BinaryCliOptions {
    /// The address in memory where the binary will be put at. This is only considered when `bin` is selected as the format.
    #[clap(long, value_parser = parse_u64, help_heading = "DOWNLOAD CONFIGURATION / BIN IMAGE")]
    pub(crate) base_address: Option<u64>,
    /// The number of bytes to skip at the start of the binary file. This is only considered when `bin` is selected as the format.
    #[clap(
        long,
        value_parser = parse_u32,
        default_value = "0",
        help_heading = "DOWNLOAD CONFIGURATION / BIN IMAGE"
    )]
    pub(crate) skip: u32,
}

/// Supported flash frequencies
///
/// Note that not all frequencies are supported by each target device.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum, Schema,
)]
#[serde(rename_all = "lowercase")]
pub enum EspFlashFrequency {
    /// 12 MHz
    #[serde(rename = "12MHz")]
    _12Mhz,
    /// 15 MHz
    #[serde(rename = "15MHz")]
    _15Mhz,
    /// 16 MHz
    #[serde(rename = "16MHz")]
    _16Mhz,
    /// 20 MHz
    #[serde(rename = "20MHz")]
    _20Mhz,
    /// 24 MHz
    #[serde(rename = "24MHz")]
    _24Mhz,
    /// 26 MHz
    #[serde(rename = "26MHz")]
    _26Mhz,
    /// 30 MHz
    #[serde(rename = "30MHz")]
    _30Mhz,
    /// 40 MHz
    #[serde(rename = "40MHz")]
    #[default]
    _40Mhz,
    /// 48 MHz
    #[serde(rename = "48MHz")]
    _48Mhz,
    /// 60 MHz
    #[serde(rename = "60MHz")]
    _60Mhz,
    /// 80 MHz
    #[serde(rename = "80MHz")]
    _80Mhz,
}

impl From<EspFlashFrequency> for espflash::flasher::FlashFrequency {
    fn from(freq: EspFlashFrequency) -> Self {
        match freq {
            EspFlashFrequency::_12Mhz => espflash::flasher::FlashFrequency::_12Mhz,
            EspFlashFrequency::_15Mhz => espflash::flasher::FlashFrequency::_15Mhz,
            EspFlashFrequency::_16Mhz => espflash::flasher::FlashFrequency::_16Mhz,
            EspFlashFrequency::_20Mhz => espflash::flasher::FlashFrequency::_20Mhz,
            EspFlashFrequency::_24Mhz => espflash::flasher::FlashFrequency::_24Mhz,
            EspFlashFrequency::_26Mhz => espflash::flasher::FlashFrequency::_26Mhz,
            EspFlashFrequency::_30Mhz => espflash::flasher::FlashFrequency::_30Mhz,
            EspFlashFrequency::_40Mhz => espflash::flasher::FlashFrequency::_40Mhz,
            EspFlashFrequency::_48Mhz => espflash::flasher::FlashFrequency::_48Mhz,
            EspFlashFrequency::_60Mhz => espflash::flasher::FlashFrequency::_60Mhz,
            EspFlashFrequency::_80Mhz => espflash::flasher::FlashFrequency::_80Mhz,
        }
    }
}

/// Supported flash modes
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum, Schema,
)]
#[serde(rename_all = "lowercase")]
pub enum EspFlashMode {
    /// Quad I/O (4 pins used for address & data)
    Qio,
    /// Quad Output (4 pins used for data)
    Qout,
    /// Dual I/O (2 pins used for address & data)
    #[default]
    Dio,
    /// Dual Output (2 pins used for data)
    Dout,
}

impl From<EspFlashMode> for espflash::flasher::FlashMode {
    fn from(mode: EspFlashMode) -> Self {
        match mode {
            EspFlashMode::Qio => espflash::flasher::FlashMode::Qio,
            EspFlashMode::Qout => espflash::flasher::FlashMode::Qout,
            EspFlashMode::Dio => espflash::flasher::FlashMode::Dio,
            EspFlashMode::Dout => espflash::flasher::FlashMode::Dout,
        }
    }
}

#[derive(clap::Parser, Clone, Serialize, Deserialize, Debug, Default, Schema)]
#[serde(default)]
pub struct IdfCliOptions {
    /// The idf bootloader path
    #[clap(long, help_heading = "DOWNLOAD CONFIGURATION / ESP-IDF IMAGE")]
    pub(crate) idf_bootloader: Option<String>,
    /// The idf partition table path
    #[clap(long, help_heading = "DOWNLOAD CONFIGURATION / ESP-IDF IMAGE")]
    pub(crate) idf_partition_table: Option<String>,
    /// The idf target app partition
    #[clap(long, help_heading = "DOWNLOAD CONFIGURATION / ESP-IDF IMAGE")]
    pub(crate) idf_target_app_partition: Option<String>,
    /// Flash SPI mode
    #[clap(long, help_heading = "DOWNLOAD CONFIGURATION / ESP-IDF IMAGE")]
    pub(crate) idf_flash_mode: Option<EspFlashMode>,
    /// Flash SPI frequency
    #[clap(long, help_heading = "DOWNLOAD CONFIGURATION / ESP-IDF IMAGE")]
    pub(crate) idf_flash_freq: Option<EspFlashFrequency>,
}

#[derive(clap::Parser, Clone, Serialize, Deserialize, Debug, Default, Schema)]
#[serde(default)]
pub struct ElfCliOptions {
    /// Section name to skip flashing. This option may be specified multiple times, and is only
    /// considered when `elf` is selected as the format.
    #[clap(long, help_heading = "DOWNLOAD CONFIGURATION / ELF IMAGE")]
    pub(crate) skip_section: Vec<String>,
}

#[derive(clap::Parser, Clone, Serialize, Deserialize, Debug, Default, Schema)]
#[serde(default)]
pub struct FormatOptions {
    /// The format of the firmware image.
    #[clap(
        value_enum,
        ignore_case = true,
        default_value_t = FormatKind::Target,
        long,
        help_heading = "DOWNLOAD CONFIGURATION"
    )]
    pub(crate) binary_format: FormatKind,

    #[clap(flatten)]
    pub(crate) bin_options: BinaryCliOptions,

    #[clap(flatten)]
    pub(crate) idf_options: IdfCliOptions,

    #[clap(flatten)]
    pub(crate) elf_options: ElfCliOptions,
}

/// A finite list of all the available binary formats probe-rs understands.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, ValueEnum, Schema)]
pub enum FormatKind {
    /// The image format is determined by the target chip's preference, which is usually ELF.
    #[default]
    Target,

    /// The image is in binary format. This means that the file contains the contents of the flash 1:1.
    #[value(alias("binary"))]
    Bin,

    /// The image is in Intel HEX format. For more information, see https://en.wikipedia.org/wiki/Intel_HEX
    #[value(aliases(["ihex", "intelhex"]))]
    Hex,

    /// The image is in the Executable and Linkable Format (ELF). For more information, see https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
    Elf,

    /// The image is an ELF file containing an ESP-IDF bootloader compatible application. For more information, see https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/app_image_format.html#app-image-structures
    #[value(aliases(["esp-idf", "espidf"]))]
    Idf,

    /// The image is in the Universal Flash Storage (UF2) format. For more information, see https://github.com/microsoft/uf2
    Uf2,
}

impl FormatKind {
    fn to_probe_rs(self, target: &Target) -> probe_rs::flashing::FormatKind {
        let this = if self == FormatKind::Target {
            FormatKind::from_optional(target.default_format.as_deref())
                .expect("Failed to parse a default binary format. This shouldn't happen.")
        } else {
            self
        };

        match this {
            FormatKind::Target => unreachable!(),
            FormatKind::Bin => probe_rs::flashing::FormatKind::Bin,
            FormatKind::Hex => probe_rs::flashing::FormatKind::Hex,
            FormatKind::Elf => probe_rs::flashing::FormatKind::Elf,
            FormatKind::Uf2 => probe_rs::flashing::FormatKind::Uf2,
            FormatKind::Idf => probe_rs::flashing::FormatKind::Idf,
        }
    }

    /// Creates a new Format from an optional string.
    ///
    /// If the string is `None`, the default format is returned.
    pub fn from_optional(s: Option<&str>) -> Result<Self, String> {
        match s {
            Some(format) => Self::from_str(format, true),
            None => Ok(Self::Elf),
        }
    }
}

impl FormatOptions {
    /// If a format is provided, use it.
    /// If a target has a preferred format, we use that.
    /// Finally, if neither of the above cases are true, we default to [`Format::default()`].
    pub fn to_format_kind(&self, target: &Target) -> probe_rs::flashing::FormatKind {
        self.binary_format.to_probe_rs(target)
    }
}
