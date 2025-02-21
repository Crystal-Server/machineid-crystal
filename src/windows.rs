use crate::errors::HWIDError;
use serde::Deserialize;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
use wmi::{COMLibrary, WMIConnection};

thread_local! {
    static COM_LIB: COMLibrary = COMLibrary::without_security().unwrap();
}

pub fn get_hwid() -> Result<String, HWIDError> {
    use winreg::enums::{KEY_READ, KEY_WOW64_64KEY};

    let rkey = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Cryptography",
        KEY_READ | KEY_WOW64_64KEY,
    )?;

    let id = rkey.get_value("MachineGuid")?;
    Ok(id)
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiskGeneric {
    serial_number: String,
}

#[derive(Deserialize)]
struct MACGeneric {
    MACAddress: String,
}

pub(crate) fn get_disk_id() -> Result<String, HWIDError> {
    let con = WMIConnection::new(COM_LIB.with(|con| *con))?;
    let ser: Vec<DiskGeneric> = con.raw_query("SELECT SerialNumber FROM Win32_PhysicalMedia")?;
    let serial = ser
        .first()
        .ok_or(HWIDError::new("UuidError", "Could not retrieve Uuid"))?
        .serial_number
        .clone();
    Ok(serial)
}

pub(crate) fn get_mac_address() -> Result<String, HWIDError> {
    let con = WMIConnection::new(COM_LIB.with(|con| *con))?;
    let ser: Vec<MACGeneric> =
        con.raw_query("SELECT MACAddress from Win32_NetworkAdapter WHERE MACAddress IS NOT NULL")?;
    Ok(ser
        .first()
        .ok_or(HWIDError::new(
            "MACAddress",
            "Could not retrieve Mac Address",
        ))?
        .MACAddress
        .clone())
}
