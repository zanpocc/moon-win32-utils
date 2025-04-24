use windows::Win32::System::Registry::{HKEY_CURRENT_USER, REG_SZ};

use windows::{
    core::PCWSTR,
    Win32::System::Registry::{
        RegCloseKey, RegCreateKeyExW, RegDeleteKeyW, RegOpenKeyExW, RegSetValueExW, HKEY,
        KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_VALUE_TYPE,
    },
};

pub struct RegistryKey {
    raw: HKEY,
}

impl RegistryKey {
    pub fn from(hkey: HKEY) -> Self {
        Self { raw: hkey }
    }

    pub fn raw(&self) -> HKEY {
        self.raw
    }
}

impl Drop for RegistryKey {
    fn drop(&mut self) {
        let _ = unsafe { RegCloseKey(self.raw) };
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_delete_registry_key() {
        let sub_key = "Software\\TestKey";
        let result = create_registry_key(HKEY_CURRENT_USER, sub_key);
        assert!(result.is_ok());

        let delete_result = delete_registry_key(HKEY_CURRENT_USER, sub_key);
        assert!(delete_result.is_ok());
    }

    #[test]
    fn test_registry_key_exists() {
        let sub_key = "Software\\TestKeyExists";
        let _ = create_registry_key(HKEY_CURRENT_USER, sub_key);

        assert!(registry_key_exists(HKEY_CURRENT_USER, sub_key));

        let _ = delete_registry_key(HKEY_CURRENT_USER, sub_key);
        assert!(!registry_key_exists(HKEY_CURRENT_USER, sub_key));
    }

    #[test]
    fn test_set_registry_value() {
        let sub_key = "Software\\TestSetValue";
        let key = create_registry_key(HKEY_CURRENT_USER, sub_key).unwrap();

        let value_name = "TestValue";
        let value_data = "TestData".as_bytes();
        let result = set_registry_value(&key, value_name, REG_SZ, value_data);
        assert!(result.is_ok());

        let _ = delete_registry_key(HKEY_CURRENT_USER, sub_key);
    }

    #[test]
    fn test_open_registry_key() {
        let sub_key = "Software\\TestOpenKey";
        let _ = create_registry_key(HKEY_CURRENT_USER, sub_key);

        let result = open_registry_key(HKEY_CURRENT_USER, sub_key);
        assert!(result.is_ok());

        let _ = delete_registry_key(HKEY_CURRENT_USER, sub_key);
    }
    
}


pub fn open_registry_key(hkey: HKEY, sub_key: &str) -> windows::core::Result<RegistryKey> {
    unsafe {
        let mut key: HKEY = HKEY::default();
        let sub_key_wide: Vec<u16> = sub_key.encode_utf16().chain([0]).collect();
        let result = RegOpenKeyExW(hkey, PCWSTR(sub_key_wide.as_ptr()), 0, KEY_WRITE, &mut key);
        if result.is_ok() {
            return Ok(RegistryKey::from(key));
        }

        return Err(windows::core::Error::from_win32());
    }
}

pub fn delete_registry_key(hkey: HKEY, sub_key: &str) -> windows::core::Result<()> {
    let sub_key_wide: Vec<u16> = sub_key.encode_utf16().chain([0]).collect();
    let r = unsafe { RegDeleteKeyW(hkey, PCWSTR(sub_key_wide.as_ptr())) };
    if r.is_err() {
        println!("delete key fault:{:?}", r);
        return Err(windows::core::Error::from_win32());
    }

    return Ok(());
}

pub fn registry_key_exists(hkey: HKEY, sub_key: &str) -> bool {
    if open_registry_key(hkey, sub_key).is_err() {
        false
    } else {
        true
    }
}

pub fn create_registry_key(hkey: HKEY, sub_key: &str) -> windows::core::Result<RegistryKey> {
    unsafe {
        let sub_key_wide: Vec<u16> = sub_key.encode_utf16().chain([0]).collect();
        let mut new_key: HKEY = HKEY::default();

        let result = RegCreateKeyExW(
            hkey,
            PCWSTR(sub_key_wide.as_ptr()),
            0,
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut new_key,
            None,
        );

        if result.is_ok() {
            Ok(RegistryKey::from(new_key))
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

pub fn set_registry_value(
    key: &RegistryKey,
    value_name: &str,
    value_type: REG_VALUE_TYPE,
    data: &[u8],
) -> windows::core::Result<()> {
    let value_name_wide: Vec<u16> = value_name.encode_utf16().chain([0]).collect();
    let result = unsafe {
        RegSetValueExW(
            key.raw,
            PCWSTR(value_name_wide.as_ptr()),
            0,
            value_type,
            Some(data),
        )
    };

    if result.is_ok() {
        Ok(())
    } else {
        Err(windows::core::Error::from_win32())
    }
}
