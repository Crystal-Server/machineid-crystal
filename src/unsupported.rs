use std::{
    fs::File,
    io::{Read, Write},
};

use uuid::Uuid;

use crate::errors::HWIDError;

macro_rules! gen_uuid {
    ($errname: expr, $androidname: expr, $globalname: expr) => {{
        #[cfg(target_os = "android")]
        {
            std::fs::create_dir_all("/storage/emulated/0/media/Machine ID Data/")
                .ok()
                .ok_or(HWIDError::new(
                    "DirError",
                    "Unable to make the MachineID Data Directory",
                ))?;
        }
        let mut f = File::create(if cfg!(target_os = "android") {
            format!("/storage/emulated/0/media/Machine ID Data/{}", $androidname)
        } else {
            String::from($globalname)
        })
        .ok()
        .ok_or(HWIDError::new(
            "FileError",
            &format!("Unable to open {} File", $errname),
        ))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        if s.is_empty() {
            s = Uuid::new_v4().to_string();
            f.write_all(s.as_bytes()).ok().ok_or(HWIDError::new(
                "FileError",
                &format!("Unable to write UUID data to {} File", $errname),
            ))?;
        }

        Ok(s)
    }};
}

pub fn get_hwid() -> Result<String, HWIDError> {
    gen_uuid!("HWID", "hwid", "__machine_id_data__hwid__")
}

pub(crate) fn get_disk_id() -> Result<String, HWIDError> {
    gen_uuid!("DiskID", "dskid", "__machine_id_data__dskid__")
}

pub(crate) fn get_mac_address() -> Result<String, HWIDError> {
    gen_uuid!("MacAddress", "mac", "__machine_id_data__mac__")
}
