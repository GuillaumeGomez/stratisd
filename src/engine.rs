// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use types::StratisResult;

pub trait Pool {
    fn add_blockdev(&mut self, path: &str) -> StratisResult<()>;
    fn add_cachedev(&mut self, path: &str) -> StratisResult<()>;
    fn destroy(&mut self) -> StratisResult<()>;
}

pub trait Engine {
    fn create_pool(&self,
                   name: &str,
                   blockdev_paths: &[&str],
                   raid_level: i32)
                   -> StratisResult<Box<Pool>>;
}
