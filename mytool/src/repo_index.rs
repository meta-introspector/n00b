//! Repository Indexing/Storage Module
//! Manages persistent storage of discovered GitHub repository metadata.

use rusqlite::{Connection, Result};
use crate::github::RepoMetadata;
use url::Url;
use log::info;
use serde_json; // Import serde_json here

pub struct RepoIndex {
    conn: Connection,
}

impl RepoIndex {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS repositories (
                id INTEGER PRIMARY KEY,
                owner TEXT NOT NULL,
                name TEXT NOT NULL,
                html_url TEXT NOT NULL UNIQUE,
                description TEXT,
                stargazers_count INTEGER,
                forks_count INTEGER,
                created_at TEXT,
                updated_at TEXT,
                pushed_at TEXT,
                language TEXT,
                license TEXT,
                topics TEXT, -- Stored as comma-separated string or JSON
                trust_score REAL DEFAULT 0.0
            )",
            (),
        )?;
        Ok(RepoIndex { conn })
    }

    pub fn insert_repo(&self, repo: &RepoMetadata) -> Result<()> {
        info!("Inserting repository: {}/{}", repo.owner, repo.name);
        self.conn.execute(
            "INSERT OR IGNORE INTO repositories (
                id, owner, name, html_url, description, stargazers_count, forks_count,
                created_at, updated_at, pushed_at, language, license, topics, trust_score
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
                repo.id,
                repo.owner,
                repo.name,
                repo.html_url.to_string(),
                repo.description,
                repo.stargazers_count,
                repo.forks_count,
                repo.created_at,
                repo.updated_at,
                repo.pushed_at,
                repo.language,
                repo.license,
                serde_json::to_string(&repo.topics).unwrap_or_default(),
                repo.trust_score, // Added trust_score here
            ],
        )?;
        Ok(())
    }

    pub fn get_all_repos(&self) -> Result<Vec<RepoMetadata>> {
        let mut stmt = self.conn.prepare("SELECT * FROM repositories")?;
        let repo_iter = stmt.query_map([], |row| {
            Ok(RepoMetadata {
                id: row.get(0)?,
                owner: row.get(1)?,
                name: row.get(2)?,
                html_url: Url::parse(&row.get::<_, String>(3)?).unwrap(),
                description: row.get(4)?,
                stargazers_count: row.get(5)?,
                forks_count: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                pushed_at: row.get(9)?,
                language: row.get(10)?,
                license: row.get(11)?,
                topics: serde_json::from_str(&row.get::<_, String>(12)?).unwrap_or_default(),
                trust_score: row.get(13)?, // ADDED THIS LINE
            })
        })?;

        let mut repos = Vec::new();
        for repo in repo_iter {
            repos.push(repo?);
        }
        Ok(repos)
    }

    pub fn update_trust_score(&self, repo_id: i64, score: f64) -> Result<()> {
        self.conn.execute(
            "UPDATE repositories SET trust_score = ?1 WHERE id = ?2",
            rusqlite::params![score, repo_id],
        )?;
        Ok(())
    }
}