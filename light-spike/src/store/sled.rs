pub mod utils;

use self::utils::*;
use crate::prelude::*;

use ::sled::Db as SledDb;

const VERIFIED_PREFIX: &str = "light_store/verified";
const UNVERIFIED_PREFIX: &str = "light_store/unverified";
const FAILED_PREFIX: &str = "light_store/failed";

/// Persistent store backed by an on-disk `sled` database.
#[derive(Debug)]
pub struct SledStore {
    db: SledDb,
    verified_db: KeyValueDb<Height, LightBlock>,
    unverified_db: KeyValueDb<Height, LightBlock>,
    failed_db: KeyValueDb<Height, LightBlock>,
}

impl SledStore {
    pub fn new(db: SledDb) -> Self {
        Self {
            db,
            verified_db: KeyValueDb::new(VERIFIED_PREFIX),
            unverified_db: KeyValueDb::new(UNVERIFIED_PREFIX),
            failed_db: KeyValueDb::new(FAILED_PREFIX),
        }
    }

    fn db(&self, status: VerifiedStatus) -> &KeyValueDb<Height, LightBlock> {
        match status {
            VerifiedStatus::Unverified => &self.unverified_db,
            VerifiedStatus::Verified => &self.verified_db,
            VerifiedStatus::Failed => &self.failed_db,
        }
    }
}

impl LightStore for SledStore {
    fn get(&self, height: Height, status: VerifiedStatus) -> Option<LightBlock> {
        self.db(status).get(&self.db, &height).ok().flatten()
    }

    fn update(&mut self, light_block: LightBlock, status: VerifiedStatus) {
        self.db(status)
            .insert(&self.db, &light_block.height(), &light_block)
            .ok();
    }

    fn insert(&mut self, light_block: LightBlock, status: VerifiedStatus) {
        self.db(status)
            .insert(&self.db, &light_block.height(), &light_block)
            .ok();
    }

    fn remove(&mut self, height: Height, status: VerifiedStatus) {
        self.db(status).remove(&self.db, &height).ok();
    }

    fn latest(&self, status: VerifiedStatus) -> Option<LightBlock> {
        // FIXME: This is very inefficient since it iterates over all the blocks in the store with the given status.
        self.all(status).max_by_key(|lb| lb.height())
    }

    fn all(&self, status: VerifiedStatus) -> Box<dyn Iterator<Item = LightBlock>> {
        Box::new(self.db(status).iter(&self.db))
    }
}
