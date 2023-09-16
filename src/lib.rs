use md5::{Md5, Digest};
use std::collections::HashSet;
use std::error::Error;
use reqwest::{Client, ClientBuilder};

use base64;
use std::cmp::Ordering;
use std::time::Duration;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Deleted {
    No,
    Soft,
    Hard,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Record {
    rvolumes: HashSet<String>,
    deleted: Deleted,
    hash: String,
}

impl Record {
    fn from_record(&self) -> String {
        let mut serialized = String::new();
        
        if self.deleted == Deleted::Soft {
            serialized.push_str("DELETED");
        }
        
        if !self.hash.is_empty() {
            serialized.push_str("HASH");
            serialized.push_str(&self.hash);
        }
        
        serialized.push_str(&self.rvolumes.iter().cloned().collect::<Vec<String>>().join(","));
        
        serialized
    }

    fn to_record(s: &str) -> Record {
        let mut s_remaining = s.to_string();
        let mut deleted = Deleted::No;
        let mut hash = String::new();
        
        if s_remaining.starts_with("DELETED") {
            deleted = Deleted::Soft;
            s_remaining = s_remaining[7..].to_string();
        }
        
        if s_remaining.starts_with("HASH") {
            hash = s_remaining[4..36].to_string();
            s_remaining = s_remaining[36..].to_string();
        }
        
        let rvolumes = s_remaining.split(',').map(|v| v.to_string()).collect::<HashSet<String>>();
        
        Record {
            rvolumes,
            deleted,
            hash,
        }
    }
}

// Convert a key to volume assignments
fn key2volume(key: &[u8], volumes: &[String], count: usize, svcount: usize) -> Vec<String> {
    let mut sortvols: Vec<SortVol> = Vec::new();

    for v in volumes.iter() {
        let mut hasher = Md5::new();
        hasher.update(key);
        hasher.update(v.as_bytes());
        let score = hasher.finalize().to_vec();
        sortvols.push(SortVol {
            score,
            volume: v.clone(),
        });
    }

    // Sort by score in descending order
    sortvols.sort();

    // Take the top 'count' volumes
    let selected_vols: Vec<String> = sortvols.iter().take(count).map(|sv| {
        let mut volume = sv.volume.clone();
        if svcount > 1 {
            let svhash = (sv.score[12] as u32) << 24
                       + (sv.score[13] as u32) << 16
                       + (sv.score[14] as u32) << 8
                       + sv.score[15] as u32;
            volume = format!("{}/sv{:02X}", sv.volume, svhash as usize % svcount);
        }
        volume
    }).collect();

    selected_vols
}


// Convert key to path
fn key2path(key: &[u8]) -> String {
    let mkey = Md5::digest(key);
    let b64key = base64::encode(key);
    format!("/{:02x}/{:02x}/{}", mkey[0], mkey[1], b64key)
}

// Structures and methods for sorting by score
struct SortVol {
    score: Vec<u8>,
    volume: String,
}

impl Ord for SortVol {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for SortVol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SortVol {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for SortVol {}

fn needs_rebalance(volumes: &[String], kvolumes: &[String]) -> bool {
    if volumes.len() != kvolumes.len() {
        return true;
    }
    for (v, kv) in volumes.iter().zip(kvolumes.iter()) {
        if v != kv {
            return true;
        }
    }
    false
}

async fn remote_delete(remote: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let resp = client.delete(remote).send().await?;
    
    match resp.status().as_u16() {
        204 | 404 => Ok(()),
        status => Err(format!("remote_delete: wrong status code {}", status).into())
    }
}

async fn remote_put(remote: &str, body: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let resp = client.put(remote).body(body).send().await?;
    
    match resp.status().as_u16() {
        201 | 204 => Ok(()),
        status => Err(format!("remote_put: wrong status code {}", status).into())
    }
}

async fn remote_get(remote: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.get(remote).send().await?;
    if resp.status().as_u16() != 200 {
        return Err(format!("remote_get: wrong status code {}", resp.status().as_u16()).into());
    }
    Ok(resp.text().await?)
}

async fn remote_head(remote: &str, timeout: Duration) -> Result<bool, Box<dyn Error>> {
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let resp = client.head(remote).send().await?;
    Ok(resp.status().as_u16() == 200)
}