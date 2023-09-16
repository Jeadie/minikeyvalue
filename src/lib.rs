use md5::{Md5, Digest};
use std::collections::{BinaryHeap, HashSet};


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
fn key2volume(key: &[u8], volumes: &[String], replicas: usize) -> Vec<String> {
    let mut hasher = Md5::new();
    hasher.update(key);
    let result = hasher.finalize();

    // Here, we'll sort the volumes based on the hash and select the required number of replicas.
    // For simplicity, we're using a binary heap to get the top-n volumes.
    let mut heap = BinaryHeap::new();
    for volume in volumes {
        let mut volume_hasher = Md5::new();
        volume_hasher.update(volume.as_bytes());
        let volume_hash = volume_hasher.finalize();
        let combined_hash = format!("{:x}{:x}", result, volume_hash);
        heap.push((combined_hash, volume));
    }
    
    let mut selected_volumes = Vec::new();
    for _ in 0..replicas {
        if let Some((_, volume)) = heap.pop() {
            selected_volumes.push(volume.to_string());
        }
    }

    selected_volumes
}
