use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

use crate::app::AppState;

use super::{Result, Tool, ToolError};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GitBackup {}

#[typetag::serde]
impl Tool for GitBackup {
    fn ui(&mut self, ui: &mut Ui, _state: &mut AppState) -> Result {
        ui.label("todo");
        Ok(())
    }
}

#[cfg(windows)]
fn get_local_app_data_low() -> std::result::Result<std::path::PathBuf, ToolError> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};

    use anyhow::Context;
    use windows::Win32::{
        Foundation::HANDLE,
        System::Com::CoTaskMemFree,
        UI::Shell::{self, KNOWN_FOLDER_FLAG},
    };

    let pwstr = unsafe {
        Shell::SHGetKnownFolderPath(
            &Shell::FOLDERID_LocalAppDataLow,
            KNOWN_FOLDER_FLAG(0),
            HANDLE::default(),
        )
    }
    .context("Failed to locate the LocalAppDataLow folder on your system somehow")?;

    let os_str = OsString::from_wide(unsafe { pwstr.as_wide() });
    unsafe { CoTaskMemFree(Some(pwstr.as_ptr() as _)) };

    Ok(PathBuf::from(os_str))
}

// #[cfg(target_os = "linux")]
// fn get_local_app_data_low() -> std::result::Result<std::path::PathBuf, ToolError> {
//     use anyhow::Context;
//     use steamlocate::SteamDir;

//     let mut steam = SteamDir::locate().context("Steam not found")?;
//     let noita_dir = steam.app(&881100).context("Noita not found")?;
//     noita_dir.path.clone()

//     Some(std::path::PathBuf::from("/var/lib"))
// }
