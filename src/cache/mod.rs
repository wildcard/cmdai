//! Cache module for managing Hugging Face model downloads and local storage
//!
//! Provides LRU cache management, integrity validation, and offline support.

use crate::models::{CacheManifest, CachedModel};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

mod manifest;
pub use manifest::ManifestManager;

/// Cache-related errors
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Failed to download model: {0}")]
    DownloadFailed(String),

    #[error("Checksum mismatch for model {model_id}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        model_id: String,
        expected: String,
        actual: String,
    },

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Manifest error: {0}")]
    ManifestError(String),

    #[error("Cache directory error: {0}")]
    DirectoryError(String),
}

/// Statistics about the cache
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cache_dir: PathBuf,
    pub total_models: usize,
    pub total_size_bytes: u64,
    pub models: Vec<String>,
}

/// Integrity validation report
#[derive(Debug, Clone)]
pub struct IntegrityReport {
    pub valid_models: Vec<String>,
    pub corrupted_models: Vec<String>,
    pub missing_models: Vec<String>,
}

/// Manages cached Hugging Face models
pub struct CacheManager {
    cache_dir: PathBuf,
    manifest: Arc<RwLock<ManifestManager>>,
}

impl CacheManager {
    /// Create a new CacheManager with default XDG cache directory
    pub fn new() -> Result<Self, CacheError> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| CacheError::DirectoryError("Could not determine cache directory".to_string()))?
            .join("cmdai")
            .join("models");

        Self::with_cache_dir(cache_dir)
    }

    /// Create a CacheManager with a custom cache directory
    pub fn with_cache_dir(cache_dir: PathBuf) -> Result<Self, CacheError> {
        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir)?;
        }

        if !cache_dir.is_dir() {
            return Err(CacheError::DirectoryError(format!(
                "Cache path is not a directory: {}",
                cache_dir.display()
            )));
        }

        let manifest = ManifestManager::new(cache_dir.clone())?;

        Ok(Self {
            cache_dir,
            manifest: Arc::new(RwLock::new(manifest)),
        })
    }

    /// Get a model from cache or download if not present
    pub async fn get_model(&self, model_id: &str) -> Result<PathBuf, CacheError> {
        // Check if model is already cached
        if self.is_cached(model_id) {
            let cached_model = {
                let manifest = self.manifest.read()
                    .map_err(|e| CacheError::ManifestError(format!("Lock error: {}", e)))?;
                manifest.get_model(model_id)
                    .ok_or_else(|| CacheError::ModelNotFound(model_id.to_string()))?
            };

            // Validate checksum
            let actual_checksum = Self::calculate_checksum(&cached_model.path).await?;
            if actual_checksum != cached_model.checksum {
                return Err(CacheError::ChecksumMismatch {
                    model_id: model_id.to_string(),
                    expected: cached_model.checksum.clone(),
                    actual: actual_checksum,
                });
            }

            // Update last accessed time
            {
                let mut manifest = self.manifest.write()
                    .map_err(|e| CacheError::ManifestError(format!("Lock error: {}", e)))?;
                manifest.update_last_accessed(model_id)?;
            }

            Ok(cached_model.path.clone())
        } else {
            // Download model (placeholder - will integrate with Hugging Face API)
            self.download_model(model_id).await
        }
    }

    /// Check if a model is cached
    pub fn is_cached(&self, model_id: &str) -> bool {
        self.manifest.read()
            .map(|manifest| manifest.has_model(model_id))
            .unwrap_or(false)
    }

    /// Remove a specific model from cache
    pub async fn remove_model(&self, model_id: &str) -> Result<(), CacheError> {
        let mut manifest = self.manifest.write()
            .map_err(|e| CacheError::ManifestError(format!("Lock error: {}", e)))?;

        let cached_model = manifest
            .get_model(model_id)
            .ok_or_else(|| CacheError::ModelNotFound(model_id.to_string()))?;

        // Delete the model file
        if cached_model.path.exists() {
            tokio::fs::remove_file(&cached_model.path).await?;
        }

        // Remove from manifest
        manifest.remove_model(model_id)?;

        Ok(())
    }

    /// Clear all cached models
    pub async fn clear_cache(&self) -> Result<(), CacheError> {
        let mut manifest = self.manifest.write()
            .map_err(|e| CacheError::ManifestError(format!("Lock error: {}", e)))?;

        // Get all model IDs before clearing
        let model_ids: Vec<String> = manifest.list_models().into_iter().map(|s| s.to_string()).collect();

        // Delete all model files
        for model_id in &model_ids {
            if let Some(cached_model) = manifest.get_model(model_id) {
                if cached_model.path.exists() {
                    tokio::fs::remove_file(&cached_model.path).await?;
                }
            }
        }

        // Clear manifest
        manifest.clear()?;

        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let (models, total_size) = self.manifest.read()
            .map(|manifest| {
                let models = manifest.list_models();
                let total_size = manifest.total_size();
                (models, total_size)
            })
            .unwrap_or_else(|_| (Vec::new(), 0));

        CacheStats {
            cache_dir: self.cache_dir.clone(),
            total_models: models.len(),
            total_size_bytes: total_size,
            models,
        }
    }

    /// Validate integrity of all cached models
    pub async fn validate_integrity(&self) -> Result<IntegrityReport, CacheError> {
        let manifest = self.manifest.read()
            .map_err(|e| CacheError::ManifestError(format!("Lock error: {}", e)))?;
        let model_ids = manifest.list_models();

        let mut valid_models = Vec::new();
        let mut corrupted_models = Vec::new();
        let mut missing_models = Vec::new();

        for model_id in model_ids {
            if let Some(cached_model) = manifest.get_model(&model_id) {
                if !cached_model.path.exists() {
                    missing_models.push(model_id.clone());
                } else {
                    match Self::calculate_checksum(&cached_model.path).await {
                        Ok(actual_checksum) => {
                            if actual_checksum == cached_model.checksum {
                                valid_models.push(model_id.clone());
                            } else {
                                corrupted_models.push(model_id.clone());
                            }
                        }
                        Err(_) => {
                            corrupted_models.push(model_id.clone());
                        }
                    }
                }
            }
        }

        Ok(IntegrityReport {
            valid_models,
            corrupted_models,
            missing_models,
        })
    }

    /// Download a model from Hugging Face (placeholder implementation)
    async fn download_model(&self, model_id: &str) -> Result<PathBuf, CacheError> {
        // Placeholder: In real implementation, this would:
        // 1. Connect to Hugging Face API
        // 2. Download model files
        // 3. Calculate checksum
        // 4. Add to manifest
        // 5. Return path

        // For now, return a DownloadFailed error indicating network requirement
        Err(CacheError::DownloadFailed(format!(
            "Model '{}' not cached and download requires network connection (not implemented yet)",
            model_id
        )))
    }

    /// Calculate SHA256 checksum of a file
    async fn calculate_checksum(path: &PathBuf) -> Result<String, CacheError> {
        use sha2::{Digest, Sha256};

        let contents = tokio::fs::read(path).await?;
        let hash = Sha256::digest(&contents);
        Ok(format!("{:x}", hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_cache_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::with_cache_dir(temp_dir.path().to_path_buf());
        assert!(cache_manager.is_ok());
    }

    #[tokio::test]
    async fn test_cache_directory_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_path = temp_dir.path().join("new_cache");

        assert!(!cache_path.exists());

        let cache_manager = CacheManager::with_cache_dir(cache_path.clone());
        assert!(cache_manager.is_ok());
        assert!(cache_path.exists());
        assert!(cache_path.is_dir());
    }

    #[test]
    fn test_is_cached_returns_false_for_missing() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();

        assert!(!cache_manager.is_cached("nonexistent-model"));
    }
}
