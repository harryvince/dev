#[derive(Copy, Clone, Debug)]
pub enum PackageManager {
    Apk,
    AptGet,
    Dnf,
    Zypper,
    Yum,
}

impl PackageManager {
    pub fn as_str(&self) -> &str {
        match self {
            PackageManager::Apk => "apk",
            PackageManager::AptGet => "apt-get",
            PackageManager::Dnf => "dnf",
            PackageManager::Zypper => "zypper",
            PackageManager::Yum => "yum",
        }
    }

    pub fn all() -> [PackageManager; 5] {
        [
            PackageManager::Apk,
            PackageManager::AptGet,
            PackageManager::Dnf,
            PackageManager::Zypper,
            PackageManager::Yum,
        ]
    }
}

