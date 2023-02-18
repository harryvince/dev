use std::process::Command;
use std::io::{Error, ErrorKind};

pub fn find_package_manager() -> Result<String, Error> {
    const PACKAGE_MANAGERS: [&str; 5] = ["apk", "apt-get", "dnf", "zypper", "yum"];

    for manager in 0..PACKAGE_MANAGERS.len() {
        let output = match Command::new(&PACKAGE_MANAGERS[manager])
            .arg("-v")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
        {
            Ok(output) => output,
            Err(_) => continue,
        };

        if output.success() {
            return Ok(PACKAGE_MANAGERS[manager].to_string());
        }
    }

    Err(Error::new(ErrorKind::NotFound, "No Package Manager found"))
}

pub fn install_required_packages() -> Result<(), Error> {

    Ok(())
}
