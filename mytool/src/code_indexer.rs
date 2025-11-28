use anyhow::{Result, Context};
use rocksdb::{DB, Options};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{Utc, DateTime};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeFileMetadata {
    pub repo_full_name: String, // e.g., "jmikedupont2/my_repo"
    pub file_path: String, // Path within the repo, e.g., "src/main.rs"
    pub file_path_hash: String, // SHA256 hash of file_path to use as part of key
    pub last_indexed_at: DateTime<Utc>,
    pub keywords_found: Vec<String>, // e.g., "mcp", "actix-web"
}

pub struct CodeIndexer {
    db: DB,
}

impl CodeIndexer {
    const CODE_METADATA_CF: &str = "code_metadata";
    const CODE_CONTENT_CF: &str = "code_content";

    pub fn new(path: PathBuf) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let cfs = vec![
            Self::CODE_METADATA_CF,
            Self::CODE_CONTENT_CF,
        ];

        let db = DB::open_cf_descriptors(&opts, path, cfs.into_iter().map(|name| {
            let cf_opts = Options::default();
            // Configure column family options here if needed
            rocksdb::ColumnFamilyDescriptor::new(name, cf_opts)
        }).collect::<Vec<_>>())
            .context("Failed to open RocksDB for CodeIndexer")?;

        Ok(Self { db })
    }

    fn generate_file_path_hash(file_path: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(file_path);
        format!("{:x}", hasher.finalize())
    }

    pub fn index_code_file(
        &self,
        owner: &str,
        repo: &str,
        file_path: &str,
        file_content: &str,
        keywords: Vec<String>,
    ) -> Result<CodeFileMetadata> {
        let repo_full_name = format!("{}/{}", owner, repo);
        let file_path_hash = Self::generate_file_path_hash(file_path);

        let metadata = CodeFileMetadata {
            repo_full_name: repo_full_name.clone(),
            file_path: file_path.to_string(),
            file_path_hash: file_path_hash.clone(),
            last_indexed_at: Utc::now(),
            keywords_found: keywords,
        };

        // Store metadata
        let cf_metadata = self.db.cf_handle(Self::CODE_METADATA_CF)
            .context("Failed to get metadata column family handle")?;
        let metadata_key = format!("{}:{}", repo_full_name, file_path_hash);
        self.db.put_cf(cf_metadata, metadata_key, serde_json::to_vec(&metadata)?)
            .context("Failed to put code file metadata into RocksDB")?;

        // Store content
        let cf_content = self.db.cf_handle(Self::CODE_CONTENT_CF)
            .context("Failed to get content column family handle")?;
        let content_key = format!("{}:{}", repo_full_name, file_path_hash);
        self.db.put_cf(cf_content, content_key, file_content.as_bytes())
            .context("Failed to put code file content into RocksDB")?;

        Ok(metadata)
    }

    pub fn get_code_file_metadata(&self, owner: &str, repo: &str, file_path: &str) -> Result<Option<CodeFileMetadata>> {
        let repo_full_name = format!("{}/{}", owner, repo);
        let file_path_hash = Self::generate_file_path_hash(file_path);
        let metadata_key = format!("{}:{}", repo_full_name, file_path_hash);

        let cf_metadata = self.db.cf_handle(Self::CODE_METADATA_CF)
            .context("Failed to get metadata column family handle")?;
        
        if let Some(data) = self.db.get_cf(cf_metadata, metadata_key)? {
            let metadata: CodeFileMetadata = serde_json::from_slice(&data)
                .context("Failed to deserialize code file metadata")?;
            Ok(Some(metadata))
        } else {
            Ok(None)
        }
    }

    pub fn get_code_file_content(&self, owner: &str, repo: &str, file_path: &str) -> Result<Option<String>> {
        let repo_full_name = format!("{}/{}", owner, repo);
        let file_path_hash = Self::generate_file_path_hash(file_path);
        let content_key = format!("{}:{}", repo_full_name, file_path_hash);

        let cf_content = self.db.cf_handle(Self::CODE_CONTENT_CF)
            .context("Failed to get content column family handle")?;

        if let Some(data) = self.db.get_cf(cf_content, content_key)? {
            let content = String::from_utf8(data)
                .context("Failed to decode code file content from UTF-8")?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    // Function to list all indexed code file metadata
    pub fn list_indexed_code_metadata(&self) -> Result<Vec<CodeFileMetadata>> {
        let cf_metadata = self.db.cf_handle(Self::CODE_METADATA_CF)
            .context("Failed to get metadata column family handle")?;
        
        let mut metadata_list = Vec::new();
        let iter = self.db.iterator_cf(cf_metadata, rocksdb::IteratorMode::Start);
        for item in iter {
            let (_, value) = item?;
            let metadata: CodeFileMetadata = serde_json::from_slice(&value)
                .context("Failed to deserialize indexed code file metadata")?;
            metadata_list.push(metadata);
        }
        Ok(metadata_list)
    }
}
