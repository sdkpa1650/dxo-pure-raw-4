//! Windows registry read helpers for Dxo Pure Raw 4.

pub const APP_KEY: &str = "Software\\Dxo\\Dxo Pure Raw 4";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Dxo Pure Raw 4";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Dxo Pure Raw 4") },
        RegValue { name: "Version",     value: String::from("v3.7.6") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Dxo Pure Raw 4") },
        RegValue { name: "Publisher",       value: String::from("Dxo") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Dxo Pure Raw 4\\uninstall.exe\"") },
    ]
}
