use std::process::Command;
use std::io::{Error, ErrorKind};

pub fn find_package_manager() -> Result<String, Error> {
    let package_managers = ["apk", "apt-get", "dnf", "zypper", "yum"];

    for manager in 0..package_managers.len() {
        let output = match Command::new(&package_managers[manager])
            .arg("-v")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
        {
            Ok(output) => output,
            Err(_) => continue,
        };

        if output.success() {
            return Ok(package_managers[manager].to_string());
        }
    }

    Err(Error::new(ErrorKind::NotFound, "No Package Manager found"))
}
