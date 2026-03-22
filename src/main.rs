use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "hozon", about = "Encrypted device backup")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Back up a device
    Backup,
    /// Restore a device from backup
    Restore,
}

/// A single entry in a backup manifest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupEntry {
    pub path: String,
    pub size: u64,
    pub hash: String,
}

/// A backup manifest with BLAKE3 integrity hash.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupManifest {
    pub entries: Vec<BackupEntry>,
    pub created_at: String,
    pub blake3_hash: String,
}

/// Create a backup manifest from a list of entries.
///
/// The `blake3_hash` is computed over the JSON-serialized entries,
/// providing tamper-evident integrity.
#[must_use]
pub fn create_manifest(entries: Vec<BackupEntry>) -> BackupManifest {
    let entries_json = serde_json::to_string(&entries).unwrap_or_default();
    let hash = blake3::hash(entries_json.as_bytes());

    BackupManifest {
        entries,
        created_at: "2026-03-22T00:00:00Z".to_string(),
        blake3_hash: hash.to_hex().to_string(),
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Backup => {
            println!("hozon: starting backup");
        }
        Commands::Restore => {
            println!("hozon: starting restore");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_manifest() {
        let manifest = create_manifest(vec![]);
        assert!(manifest.entries.is_empty());
        assert!(!manifest.blake3_hash.is_empty());
    }

    #[test]
    fn single_entry() {
        let entry = BackupEntry {
            path: "/data/app/com.example.apk".to_string(),
            size: 1024,
            hash: "abc123".to_string(),
        };
        let manifest = create_manifest(vec![entry.clone()]);
        assert_eq!(manifest.entries.len(), 1);
        assert_eq!(manifest.entries[0], entry);
    }

    #[test]
    fn hash_determinism() {
        let entries = vec![BackupEntry {
            path: "/sdcard/photo.jpg".to_string(),
            size: 2048,
            hash: "deadbeef".to_string(),
        }];
        let m1 = create_manifest(entries.clone());
        let m2 = create_manifest(entries);
        assert_eq!(m1.blake3_hash, m2.blake3_hash);
    }

    #[test]
    fn multiple_entries_size_sum() {
        let entries = vec![
            BackupEntry {
                path: "a.txt".to_string(),
                size: 100,
                hash: "h1".to_string(),
            },
            BackupEntry {
                path: "b.txt".to_string(),
                size: 200,
                hash: "h2".to_string(),
            },
            BackupEntry {
                path: "c.txt".to_string(),
                size: 300,
                hash: "h3".to_string(),
            },
        ];
        let manifest = create_manifest(entries);
        let total_size: u64 = manifest.entries.iter().map(|e| e.size).sum();
        assert_eq!(total_size, 600);
        assert_eq!(manifest.entries.len(), 3);
    }

    #[test]
    fn manifest_serialization() {
        let entries = vec![BackupEntry {
            path: "test.apk".to_string(),
            size: 512,
            hash: "cafe".to_string(),
        }];
        let manifest = create_manifest(entries);
        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: BackupManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest, deserialized);
    }
}
