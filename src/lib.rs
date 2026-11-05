#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionRecipe {
    pub mount_point: String,
    pub fs_type: String,
    pub size_mb: u64,
}

#[derive(Debug, Default)]
pub struct InstallationGuideValidator {
    recipes: Vec<PartitionRecipe>,
}

impl InstallationGuideValidator {
    pub fn new() -> Self { Self { recipes: Vec::new() } }
    pub fn add_partition(&mut self, mount_point: &str, fs_type: &str, size_mb: u64) {
        self.recipes.push(PartitionRecipe {
            mount_point: mount_point.to_string(),
            fs_type: fs_type.to_string(),
            size_mb,
        });
    }
    pub fn has_efi_boot(&self) -> bool {
        self.recipes.iter().any(|r| r.mount_point == "/boot/efi" && r.fs_type == "fat32" && r.size_mb >= 512)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_installation_guide_validator_aaa() {
        // Arrange
        let mut validator = InstallationGuideValidator::new();
        // Act
        validator.add_partition("/boot/efi", "fat32", 512);
        validator.add_partition("/", "ext4", 51200);
        // Assert
        assert!(validator.has_efi_boot());
    }
}
