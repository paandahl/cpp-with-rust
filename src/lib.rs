use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::pin::Pin;

#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {

    // Shared structs with fields visible to both languages.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type BlobstoreClient;
        fn new_blobstore_client() -> Box<BlobstoreClient>;
        fn put(&mut self, parts: Pin<&mut MultiBuf>) -> u64;
        fn tag(&mut self, blobid: u64, tag: String);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }

    // C++ types and signatures exposed to  Rust.
    unsafe extern "C++" {
        include!("multibuf.h");
        type MultiBuf;
        fn has_more_chunks(&self) -> bool;
        fn next_chunk(self: Pin<&mut MultiBuf>) -> Pin<&mut CxxVector<u8>>;
    }
}

use ffi::*;

fn new_blobstore_client() -> Box<BlobstoreClient> {
    Box::new(BlobstoreClient { blobs: HashMap::new() })
}

struct Blob {
    data: Vec<u8>,
    tags: HashSet<String>,
}

struct BlobstoreClient {
    blobs: HashMap<u64, Blob>,
}

impl BlobstoreClient {
    fn put(&mut self, mut parts: Pin<&mut MultiBuf>) -> u64 {
        let mut contents = Vec::<u8>::new();
        while parts.has_more_chunks() {
            let chunk = parts.as_mut().next_chunk();
            contents.extend_from_slice(chunk.as_ref().as_slice());
        }
        let mut hasher = DefaultHasher::new();
        contents.hash(&mut hasher);
        let hash = hasher.finish();
        self.blobs.insert(hash, Blob { data: Vec::new(), tags: HashSet::new() });
        hash
    }

    fn tag(&mut self, blobid: u64, tag: String) {
        self.blobs.get_mut(&blobid).unwrap().tags.insert(tag);
    }

    // Retrieve metadata about a blob.
    fn metadata(&self, blobid: u64) -> BlobMetadata {
        let blob = self.blobs.get(&blobid).unwrap();
        BlobMetadata { size: blob.data.len(), tags: blob.tags.clone().into_iter().collect() }
    }
}
