use std::process::Command;
use std::io::{Error, ErrorKind};
use crate::api::types::PackageManager;

pub fn find_package_manager() -> Result<PackageManager, Error> {
    for manager in PackageManager::all().iter() {
        let output = match Command::new(manager.as_str())
            .arg("-v")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            {
                Ok(output) => output,
                Err(_) => continue,
            };

        if output.success() {
            return Ok(manager.clone());
        }
    }
    Err(Error::new(ErrorKind::NotFound, "No Package Manager found"))
}

pub fn gain_sudo() -> Result<(), Error> {
    match Command::new("sudo")
        .arg("-v")
        .status()
        {
            Ok(_) => return Ok(()),
            Err(_) => return Err(Error::new(ErrorKind::NotFound, "No Package Manager found")),
        };
}
