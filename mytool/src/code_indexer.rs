use anyhow::{Result, Context};
use rocksdb::{DB, Options};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{Utc, DateTime};
use sha2::{Sha256, Digest};

/// Metadata associated with an indexed code file.
///
/// This struct stores essential information about a code file that has been
/// processed and stored by the `CodeIndexer`, including its repository,
/// path, last indexing time, and extracted keywords.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeFileMetadata {
    /// The full name of the repository (e.g., "owner/repo_name").
    pub repo_full_name: String,
    /// The path to the file within its repository (e.g., "src/main.rs").
    pub file_path: String,
    /// A SHA256 hash of the `file_path`, used as part of the key for storage.
    pub file_path_hash: String,
    /// The UTC timestamp when this file was last indexed.
    pub last_indexed_at: DateTime<Utc>,
    /// A list of keywords extracted from the file's content, useful for search and categorization.
    pub keywords_found: Vec<String>,
}

/// A service responsible for indexing and retrieving code file content and metadata using RocksDB.
///
/// The `CodeIndexer` stores code files and their associated metadata in a persistent
/// key-value store, enabling efficient retrieval and analysis. It uses separate
/// [Column Families](https://github.com/facebook/rocksdb/wiki/Column-Families) for metadata and content.
pub struct CodeIndexer {
    db: DB,
}

impl CodeIndexer {
    /// The name of the RocksDB Column Family for storing code file metadata.
    const CODE_METADATA_CF: &str = "code_metadata";
    /// The name of the RocksDB Column Family for storing code file content.
    const CODE_CONTENT_CF: &str = "code_content";

    /// Creates a new `CodeIndexer` instance, opening or creating the RocksDB database.
    ///
    /// This function initializes the RocksDB instance at the specified `path`, ensuring
    /// that necessary [Column Families](https://github.com/facebook/rocksdb/wiki/Column-Families)
    /// for metadata and content exist.
    ///
    /// # Arguments
    /// * `path` - The file system path where the RocksDB database should be stored.
    ///
    /// # Returns
    /// A `Result` containing the `CodeIndexer` instance or an `anyhow::Error` if the
    /// database cannot be opened or created.
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

    /// Generates a SHA256 hash of a file path.
    ///
    /// This hash is used to create a stable and unique identifier for a file path
    /// within the RocksDB keys, which helps in efficient lookup and prevents issues
    /// with long or problematic file paths.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file (e.g., "src/main.rs").
    ///
    /// # Returns
    /// A hexadecimal string representing the SHA256 hash of the input `file_path`.
    fn generate_file_path_hash(file_path: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(file_path);
        format!("{:x}", hasher.finalize())
    }

    /// Indexes a single code file, storing both its content and metadata.
    ///
    /// This method fetches the given code file's content from a source (implicitly
    /// expected to be provided as `file_content`), extracts relevant keywords,
    /// and stores both the `CodeFileMetadata` and the `file_content` in RocksDB.
    ///
    /// # Arguments
    /// * `owner` - The owner of the repository.
    /// * `repo` - The name of the repository.
    /// * `file_path` - The path to the file within the repository.
    /// * `file_content` - The actual content of the code file.
    /// * `keywords` - A list of keywords to associate with the file.
    ///
    /// # Returns
    /// A `Result` containing the `CodeFileMetadata` of the indexed file or an
    /// `anyhow::Error` if storage fails.
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

    /// Retrieves the `CodeFileMetadata` for a specific code file.
    ///
    /// # Arguments
    /// * `owner` - The owner of the repository.
    /// * `repo` - The name of the repository.
    /// * `file_path` - The path to the file within the repository.
    ///
    /// # Returns
    /// A `Result` containing an `Option` of `CodeFileMetadata`. `None` if the
    /// file's metadata is not found.
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

    /// Retrieves the content of a specific indexed code file.
    ///
    /// # Arguments
    /// * `owner` - The owner of the repository.
    /// * `repo` - The name of the repository.
    /// * `file_path` - The path to the file within the repository.
    ///
    /// # Returns
    /// A `Result` containing an `Option` of the file's content as a `String`.
    /// `None` if the file's content is not found.
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

    /// Lists all indexed code file metadata.
    ///
    /// This method iterates through the `code_metadata` Column Family and
    /// collects all stored `CodeFileMetadata`.
    ///
    /// # Returns
    /// A `Result` containing a `Vec` of all `CodeFileMetadata` instances,
    /// or an `anyhow::Error` if deserialization or database iteration fails.
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
