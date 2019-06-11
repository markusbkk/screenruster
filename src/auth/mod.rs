// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// This file is part of screenruster.
//
// screenruster is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// screenruster is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with screenruster.  If not, see <http://www.gnu.org/licenses/>.

use crate::error;

pub trait Authenticate: Send + 'static {
	fn authenticate(&mut self, user: &str, password: &str) -> error::Result<bool>;
}

mod auth;
pub use self::auth::{Auth, Request, Response};

#[cfg(feature = "auth-internal")]
mod internal;

#[cfg(feature = "auth-pam")]
mod pam;
