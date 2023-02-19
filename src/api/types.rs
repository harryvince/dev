use std::io::{Error, ErrorKind};
use std::process::Command;

#[derive(Copy, Clone, Debug)]
pub enum PackageManager {
    Apk,
    AptGet,
    Dnf,
    Zypper,
    Yum,
    Homebrew,
}

impl PackageManager {
    pub fn as_str(&self) -> &str {
        match self {
            PackageManager::Apk => "apk",
            PackageManager::AptGet => "apt-get",
            PackageManager::Dnf => "dnf",
            PackageManager::Zypper => "zypper",
            PackageManager::Yum => "yum",
            PackageManager::Homebrew => "brew",
        }
    }

    pub fn all() -> [PackageManager; 6] {
        [
            PackageManager::Apk,
            PackageManager::AptGet,
            PackageManager::Dnf,
            PackageManager::Zypper,
            PackageManager::Yum,
            PackageManager::Homebrew,
        ]
    }

    pub fn install_packages(&self, packages: Vec<&str>) -> Result<(), Error> {
        match self {
            PackageManager::Apk => {
                match Command::new("apk")
                    .arg("update")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    {
                        Ok(_) => println!("Successfully updated package manager."),
                        Err(_) => return Err(Error::new(ErrorKind::NotFound, "No Package Manager found")),
                    };
                return Ok(())
            },
            PackageManager::AptGet => {
                println!("Updating your package manager...");
                match Command::new("sudo")
                    .arg("apt-get")
                    .arg("update")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    {
                        Ok(_) => println!("Successfully updated package manager."),
                        Err(_) => return Err(Error::new(ErrorKind::NotFound, "No Package Manager found")),
                    };
                return Ok(())
            },
            PackageManager::Dnf => {
                println!("Updating your package manager...");
                match Command::new("dnf")
                    .arg("check-update")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    {
                        Ok(_) => println!("Successfully updated package manager."),
                        Err(_) => return Err(Error::new(ErrorKind::NotFound, "No Package Manager found")),
                    };
                return Ok(())
            },

            PackageManager::Zypper => {
                println!("Updating your package manager...");
                match Command::new("zypper")
                    .arg("refresh")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    {
                        Ok(_) => println!("Successfully updated package manager."),
                        Err(_) => return Err(Error::new(ErrorKind::NotFound, "No Package Manager found")),
                    };
                return Ok(())
            },
            PackageManager::Yum => {
                println!("Updating your package manager...");
                match Command::new("yum")
                    .arg("check-update")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    {
                        Ok(_) => println!("Successfully updated package manager."),
                        Err(_) => return Err(Error::new(ErrorKind::NotFound, "No Package Manager found")),
                    };
                return Ok(())
            },
            PackageManager::Homebrew => {
                // I'm not sure if you can do this process in homebrew, if you can
                // please add it
                println!("No need to update, mac user :)");
                Ok(())
            },
        }

    }
}

