/// This module implements the googletest test sharding protocol.  The Google
/// sharding protocol consists of the following environment variables:
///
/// * GTEST_TOTAL_SHARDS: total number of shards.
/// * GTEST_SHARD_INDEX: number of this shard
/// * GTEST_SHARD_STATUS_FILE: touch this file to indicate support for sharding.
///
/// See also <https://google.github.io/googletest/advanced.html>
use std::cell::OnceCell;
use std::env::{var, var_os};
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};

/// Environment variable specifying the total number of test shards.
const TEST_TOTAL_SHARDS: &[&str] = &["GTEST_TOTAL_SHARDS", "TEST_TOTAL_SHARDS"];

/// Environment variable specifyign the index of this test shard.
const TEST_SHARD_INDEX: &[&str] = &["GTEST_SHARD_INDEX", "TEST_SHARD_INDEX"];

/// Environment variable specifying the name of the file we create (or cause a
/// timestamp change on) to indicate that we support the sharding protocol.
const TEST_SHARD_STATUS_FILE: &[&str] = &["GTEST_SHARD_STATUS_FILE", "TEST_SHARD_STATUS_FILE"];

thread_local! {
    static SHARDING: OnceCell<Sharding> = const { OnceCell::new() };
}

struct Sharding {
    this_shard: u64,
    total_shards: NonZeroU64,
}

impl Default for Sharding {
    fn default() -> Self {
        Self { this_shard: 0, total_shards: NonZeroU64::MIN }
    }
}

pub fn test_should_run(test_case_hash: u64) -> bool {
    SHARDING.with(|sharding_cell| {
        sharding_cell.get_or_init(Sharding::from_environment).test_should_run(test_case_hash)
    })
}

fn get_var(keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Ok(value) = var(OsStr::new(key)) {
            return Some(value);
        }
    }

    None
}

fn get_var_os(keys: &[&str]) -> Option<OsString> {
    for key in keys {
        if let Some(value) = var_os(OsStr::new(key)) {
            return Some(value);
        }
    }

    None
}

impl Sharding {
    fn test_should_run(&self, test_case_hash: u64) -> bool {
        (test_case_hash % self.total_shards.get()) == self.this_shard
    }

    fn from_environment() -> Sharding {
        let this_shard: Option<u64> =
            { get_var(TEST_SHARD_INDEX).and_then(|value| value.parse().ok()) };
        let total_shards: Option<NonZeroU64> = {
            get_var(TEST_TOTAL_SHARDS)
                .and_then(|value| value.parse().ok())
                .and_then(NonZeroU64::new)
        };

        match (this_shard, total_shards) {
            (Some(this_shard), Some(total_shards)) if this_shard < total_shards.get() => {
                if let Some(name) = get_var_os(TEST_SHARD_STATUS_FILE) {
                    let pathbuf = PathBuf::from(name);
                    if let Err(e) = create_status_file(&pathbuf) {
                        eprintln!(
                            "failed to create $GTEST_SHARD_STATUS_FILE file {}: {}",
                            pathbuf.display(),
                            e
                        );
                    }
                }

                Sharding { this_shard, total_shards }
            }
            _ => Sharding::default(),
        }
    }
}

fn create_status_file(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    File::create(path).map(|_| ())
}
