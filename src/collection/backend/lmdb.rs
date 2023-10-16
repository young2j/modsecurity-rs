// Copyright 2023 young2j
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{env::temp_dir, path::Path, sync::Arc};

use lmdb_zero::{
    open::Flags as OpenFlags,
    put::Flags as PutFlags,
    traits::{LmdbOrdKey, LmdbRaw},
    Database, DatabaseOptions, EnvBuilder, Environment, ReadTransaction, WriteTransaction,
};

use crate::{
    collection::Collection,
    variables::{KeyExclusions, VariableValue},
};

pub struct LMDB {
    m_name: String,
    db: Database<'static>,
    w_txn: WriteTransaction<'static>,
    r_txn: ReadTransaction<'static>,
    env: Arc<Environment>,
}

impl LMDB {
    pub fn into_env(&self) -> &Environment {
        self.env.as_ref()
    }
}

impl Collection for LMDB {
    fn new(name: &str) -> LMDB {
        let path_buf = temp_dir().join(Path::new(name));
        let db_path = path_buf.to_str().unwrap();
        let env = unsafe {
            let env = EnvBuilder::new()
                .unwrap()
                .open(db_path, OpenFlags::empty(), 0o600)
                .unwrap();

            Arc::new(env)
        };

        let opts = DatabaseOptions::create_multimap::<LmdbKV, LmdbKV>();
        let db = Database::open(env.clone(), Some(name), &opts).unwrap();
        let w_txn = WriteTransaction::new(env.clone()).unwrap();
        let r_txn = ReadTransaction::new(env.clone()).unwrap();

        LMDB {
            m_name: name.to_string(),
            db,
            w_txn,
            r_txn,
            env,
        }
    }

    fn store(&mut self, key: String, value: String) {
        let mut access = self.w_txn.access();
        _ = access.put(
            &self.db,
            &LmdbKV::new(key.as_str()),
            &LmdbKV::new(value.as_str()),
            PutFlags::empty(),
        );
    }

    fn update_first(&mut self, key: &str, value: &str) -> bool {
        let mut ok = false;
        let k = LmdbKV::from(key);
        let v = LmdbKV::from(value);

        let r_access = self.r_txn.access();
        let first_item = r_access.get::<LmdbKV, LmdbKV>(&self.db, &k);
        if let Ok(val) = first_item {
            let mut w_access = self.w_txn.access();
            _ = w_access.del_item(&self.db, &k, val);
            _ = w_access.put(&self.db, &k, &v, PutFlags::empty());
            ok = true;
        };

        ok
    }

    fn store_or_update_first(&mut self, key: &str, value: &str) -> bool {
        if !self.update_first(key, value) {
            self.store(key.to_string(), value.to_string());
        }

        return true;
    }

    fn del(&mut self, key: &str) {
        let mut access = self.w_txn.access();
        _ = access.del_key(&self.db, &LmdbKV::from(key));
    }

    fn resolve_first(&self, key: &str) -> Option<&str> {
        let r_access = self.r_txn.access();
        let first_item = r_access.get::<LmdbKV, LmdbKV>(&self.db, &LmdbKV::from(key));
        match first_item {
            Ok(val) => Some(val.data),
            _ => None,
        }
    }

    fn resolve_single_match(&self, key: &str, mut l: Vec<VariableValue>) {
        let k = LmdbKV::from(key);
        let r_access = self.r_txn.access();
        let mut cursor = self.r_txn.cursor(&self.db).unwrap();
        _ = cursor.seek_k::<LmdbKV, LmdbKV>(&r_access, &k);
        loop {
            let item = cursor.get_multiple::<LmdbKV>(&r_access);
            match item {
                Ok(val) => l.push(VariableValue::new_with_collection(
                    &self.m_name,
                    key,
                    val.data,
                )),
                _ => break,
            }
        }
    }

    fn resolve_multi_matches(&self, key: &str, mut l: Vec<VariableValue>, ke: KeyExclusions) {
        if key.len() == 0 {
            let r_access = self.r_txn.access();
            let mut cursor = self.r_txn.cursor(&self.db).unwrap();
            let first_item = cursor.first::<LmdbKV, LmdbKV>(&r_access);
            match first_item {
                Err(_) => return,
                Ok((k, v)) => {
                    if !ke.to_omit(k.data) {
                        l.insert(
                            0,
                            VariableValue::new_with_collection(&self.m_name, k.data, v.data),
                        );
                    }
                }
            }
            loop {
                let next_item = cursor.next::<LmdbKV, LmdbKV>(&r_access);
                match next_item {
                    Err(_) => break,
                    Ok((k, v)) => {
                        if !ke.to_omit(k.data) {
                            l.insert(
                                0,
                                VariableValue::new_with_collection(&self.m_name, k.data, v.data),
                            );
                        }
                    }
                }
            }
        } else {
            let r_access = self.r_txn.access();
            let mut cursor = self.r_txn.cursor(&self.db).unwrap();
            _ = cursor.seek_k::<LmdbKV, LmdbKV>(&r_access, &LmdbKV::from(key));
            let first_item = cursor.first_dup::<LmdbKV>(&r_access);
            match first_item {
                Err(_) => return,
                Ok(val) => {
                    if !ke.to_omit(key) {
                        l.insert(
                            0,
                            VariableValue::new_with_collection(&self.m_name, key, val.data),
                        );
                    }
                }
            }
            loop {
                let next_item = cursor.next_dup::<LmdbKV, LmdbKV>(&r_access);
                match next_item {
                    Err(_) => break,
                    Ok((k, v)) => {
                        if !ke.to_omit(k.data) {
                            l.insert(
                                0,
                                VariableValue::new_with_collection(&self.m_name, k.data, v.data),
                            );
                        }
                    }
                }
            }
        }
    }

    fn resolve_regular_expression(
        &self,
        re_key: &str,
        mut l: Vec<VariableValue>,
        ke: KeyExclusions,
    ) {
        let r_access = self.r_txn.access();
        let mut cursor = self.r_txn.cursor(&self.db).unwrap();
        let first_item = cursor.first::<LmdbKV, LmdbKV>(&r_access);
        match first_item {
            Err(_) => return,
            Ok((k, v)) => {
                if !ke.to_omit(k.data) {
                    // && regex_search(re_key)
                    l.insert(
                        0,
                        VariableValue::new_with_collection(&self.m_name, k.data, v.data),
                    );
                }
            }
        }

        loop {
            let next_item = cursor.next::<LmdbKV, LmdbKV>(&r_access);
            match next_item {
                Err(_) => break,
                Ok((k, v)) => {
                    if !ke.to_omit(k.data) {
                        // && regex_search(re_key)
                        l.insert(
                            0,
                            VariableValue::new_with_collection(&self.m_name, k.data, v.data),
                        );
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct LmdbKV<'a> {
    size: usize,
    data: &'a str,
}
unsafe impl LmdbOrdKey for LmdbKV<'_> {}
unsafe impl LmdbRaw for LmdbKV<'_> {}
impl<'a> From<&'a str> for LmdbKV<'a> {
    fn from(value: &'a str) -> Self {
        LmdbKV {
            size: value.len(),
            data: value,
        }
    }
}

impl<'a> LmdbKV<'a> {
    fn new(data: &'a str) -> LmdbKV {
        LmdbKV {
            size: data.len(),
            data,
        }
    }
}
