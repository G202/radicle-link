// This file is part of radicle-link
// <https://github.com/radicle-dev/radicle-link>
//
// Copyright (C) 2019-2020 The Radicle Team <dev@radicle.xyz>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 3 or
// later as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use thiserror::Error;

use super::{
    super::{storage, types::reference},
    local,
};
use crate::identities::{
    self,
    git::{Urn, VerificationError},
};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("the URN {0} does not exist")]
    NotFound(Urn),

    #[error("malformed URN")]
    Ref(#[from] reference::FromUrnError),

    #[error(transparent)]
    LocalId(#[from] local::ValidationError),

    #[error(transparent)]
    Verification(#[from] VerificationError),

    #[error(transparent)]
    Config(#[from] storage::config::Error),

    #[error(transparent)]
    Storage(#[from] storage::Error),

    #[error(transparent)]
    Verify(#[from] identities::git::error::Verify),

    #[error(transparent)]
    Merge(#[from] identities::git::error::Merge),

    #[error(transparent)]
    Load(#[from] identities::git::error::Load),

    #[error(transparent)]
    Store(#[from] identities::git::error::Store),

    #[error(transparent)]
    Git(#[from] git2::Error),
}