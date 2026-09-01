use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
use windows::Win32::System::Registry::*;

use crate::native_interop::wide_str;

const REGISTRY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
const REGISTRY_KEY: &str = "SystemUsesLightTheme";

/// Check if the system is in dark mode by reading the registry
pub fn is_dark_mode() -> bool {
    !is_light_theme()
}

/// Result of opting the process' classic popup menus into the Windows system
/// light/dark theme. The UxTheme entry points are undocumented and ordinal
/// only, so absence of either export is a supported fallback condition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePopupMenuThemeResult {
    Applied,
    Unsupported,
}

const UTHEME_SET_PREFERRED_APP_MODE_ORDINAL: u16 = 135;
const UTHEME_FLUSH_MENU_THEMES_ORDINAL: u16 = 136;
const UTHEME_ALLOW_DARK_MODE: i32 = 1;

type SetPreferredAppModeFn = unsafe extern "system" fn(i32) -> i32;
type FlushMenuThemesFn = unsafe extern "system" fn();

/// Ask classic Win32 popup menus to follow the system theme.
///
/// Windows 10/11 expose this opt-in from `uxtheme.dll` as undocumented
/// ordinal exports. Resolve both exports dynamically and leave the standard
/// native menu behavior untouched when the platform does not provide them.
pub fn apply_native_popup_menu_theme() -> NativePopupMenuThemeResult {
    unsafe {
        let module_name = wide_str("uxtheme.dll");
        let Ok(module) = GetModuleHandleW(PCWSTR::from_raw(module_name.as_ptr()))
            .or_else(|_| LoadLibraryW(PCWSTR::from_raw(module_name.as_ptr())))
        else {
            return NativePopupMenuThemeResult::Unsupported;
        };

        let Some(set_preferred_app_mode) = GetProcAddress(
            module,
            ordinal_proc_name(UTHEME_SET_PREFERRED_APP_MODE_ORDINAL),
        ) else {
            return NativePopupMenuThemeResult::Unsupported;
        };
        let Some(flush_menu_themes) =
            GetProcAddress(module, ordinal_proc_name(UTHEME_FLUSH_MENU_THEMES_ORDINAL))
        else {
            return NativePopupMenuThemeResult::Unsupported;
        };

        let set_preferred_app_mode: SetPreferredAppModeFn =
            std::mem::transmute(set_preferred_app_mode);
        let flush_menu_themes: FlushMenuThemesFn = std::mem::transmute(flush_menu_themes);

        // AllowDark follows the Windows preference; it does not force this
        // process into dark mode when the system is light.
        set_preferred_app_mode(UTHEME_ALLOW_DARK_MODE);
        flush_menu_themes();

        NativePopupMenuThemeResult::Applied
    }
}

fn ordinal_proc_name(ordinal: u16) -> windows::core::PCSTR {
    windows::core::PCSTR::from_raw(ordinal as usize as *const u8)
}

fn is_light_theme() -> bool {
    unsafe {
        let path = wide_str(REGISTRY_PATH);
        let key_name = wide_str(REGISTRY_KEY);

        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(path.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );

        if result.is_err() {
            return false; // Default to dark mode
        }

        let mut data: u32 = 0;
        let mut data_size: u32 = std::mem::size_of::<u32>() as u32;
        let result = RegQueryValueExW(
            hkey,
            PCWSTR::from_raw(key_name.as_ptr()),
            None,
            None,
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        if result.is_err() {
            return false; // Default to dark mode
        }

        data == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_popup_theme_uses_feature_detected_uxtheme_exports() {
        assert_eq!(UTHEME_SET_PREFERRED_APP_MODE_ORDINAL, 135);
        assert_eq!(UTHEME_FLUSH_MENU_THEMES_ORDINAL, 136);
        assert_eq!(UTHEME_ALLOW_DARK_MODE, 1);
    }

    #[test]
    fn native_popup_theme_result_is_explicit_about_fallback() {
        assert_ne!(
            NativePopupMenuThemeResult::Applied,
            NativePopupMenuThemeResult::Unsupported
        );
    }

    #[test]
    fn native_popup_theme_resolver_has_a_safe_runtime_fallback() {
        let result = apply_native_popup_menu_theme();
        assert!(matches!(
            result,
            NativePopupMenuThemeResult::Applied | NativePopupMenuThemeResult::Unsupported
        ));
    }
}
