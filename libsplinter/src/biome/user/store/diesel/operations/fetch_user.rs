// Copyright 2018-2020 Cargill Incorporated
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

use super::UserStoreOperations;
use crate::biome::user::store::diesel::models::UserModel;
use crate::biome::user::store::diesel::schema::splinter_user;
use crate::biome::user::store::error::UserStoreError;
use crate::biome::user::store::User;

use diesel::{prelude::*, result::Error::NotFound};

pub(in crate::biome::user) trait UserStoreFetchUserOperation {
    fn fetch_user(&self, user_id: &str) -> Result<User, UserStoreError>;
}

impl<'a, C> UserStoreFetchUserOperation for UserStoreOperations<'a, C>
where
    C: diesel::Connection,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, C::Backend>,
{
    fn fetch_user(&self, user_id: &str) -> Result<User, UserStoreError> {
        let user = splinter_user::table
            .find(user_id)
            .first::<UserModel>(self.conn)
            .map(Some)
            .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
            .map_err(|err| UserStoreError::OperationError {
                context: "Failed to fetch user".to_string(),
                source: Box::new(err),
            })?
            .ok_or_else(|| {
                UserStoreError::NotFoundError(format!("Failed to find user: {}", user_id))
            })?;
        Ok(User::from(user))
    }
}
