use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BtPeer {
    pub info_hash: String,
    pub ip: String,
    pub port: u16,
    pub left: Option<u64>,
    pub last_seen: i64,
}

pub struct PeerStore {
    peers: Arc<RwLock<HashMap<String, HashMap<String, BtPeer>>>>,
}

impl PeerStore {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn upsert_peer(
        &self,
        info_hash: &str,
        ip: &str,
        port: u16,
        left: Option<u64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("{}:{}", ip, port);
        let now = now_ts();
        let mut guard = self.peers.write().await;
        let by_hash = guard
            .entry(info_hash.to_string())
            .or_insert_with(HashMap::new);

        if let Some(existing) = by_hash.get_mut(&key) {
            existing.last_seen = now;
            if left.is_some() {
                existing.left = left;
            }
        } else {
            by_hash.insert(
                key,
                BtPeer {
                    info_hash: info_hash.to_string(),
                    ip: ip.to_string(),
                    port,
                    left,
                    last_seen: now,
                },
            );
        }
        Ok(())
    }

    pub async fn remove_peer(
        &self,
        info_hash: &str,
        ip: &str,
        port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("{}:{}", ip, port);
        let mut guard = self.peers.write().await;
        if let Some(by_hash) = guard.get_mut(info_hash) {
            by_hash.remove(&key);
            if by_hash.is_empty() {
                guard.remove(info_hash);
            }
        }
        Ok(())
    }

    pub async fn list_peers(
        &self,
        info_hash: &str,
        limit: usize,
    ) -> Result<Vec<BtPeer>, Box<dyn std::error::Error>> {
        let guard = self.peers.read().await;
        let out = guard
            .get(info_hash)
            .map(|m| m.values().take(limit).cloned().collect())
            .unwrap_or_default();
        Ok(out)
    }

    pub async fn peer_stats(
        &self,
        info_hash: &str,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        let guard = self.peers.read().await;
        let Some(by_hash) = guard.get(info_hash) else {
            return Ok((0, 0));
        };

        let mut complete = 0usize;
        let mut incomplete = 0usize;
        for peer in by_hash.values() {
            if peer.left == Some(0) {
                complete += 1;
            } else {
                incomplete += 1;
            }
        }
        Ok((complete, incomplete))
    }

    pub async fn list_all_infohashes(
        &self,
        limit: usize,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let guard = self.peers.read().await;
        Ok(guard.keys().take(limit).cloned().collect())
    }

    pub async fn prune_old_peers(
        &self,
        cutoff_timestamp: i64,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let mut removed = 0u64;
        let mut guard = self.peers.write().await;
        let mut empty_hashes = Vec::new();

        for (info_hash, by_hash) in guard.iter_mut() {
            let before = by_hash.len();
            by_hash.retain(|_, peer| peer.last_seen >= cutoff_timestamp);
            removed += (before - by_hash.len()) as u64;
            if by_hash.is_empty() {
                empty_hashes.push(info_hash.clone());
            }
        }

        for ih in empty_hashes {
            guard.remove(&ih);
        }

        Ok(removed)
    }
}

fn now_ts() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
