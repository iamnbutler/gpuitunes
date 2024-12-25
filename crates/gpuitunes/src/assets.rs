use anyhow::{anyhow, Result};
use derive_static_str::{static_str, DeriveStaticStr};
use gpui::*;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "svg/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| {
                if p.starts_with(path) {
                    Some(p.into())
                } else {
                    None
                }
            })
            .collect())
    }
}

#[derive(Debug, EnumIter, EnumString, IntoStaticStr, Clone, Copy, DeriveStaticStr)]
#[static_str(prefix = "svg/", suffix = ".svg")]
#[strum(serialize_all = "snake_case")]
pub enum Icon {
    Eye,
    MagnifyingGlass,
    Next,
    Pause,
    Previous,
    VolumeHigh,
    VolumeLow,
    XCircle,
}

impl Icon {
    pub fn path(&self) -> &'static str {
        self.as_static_str()
    }
}
