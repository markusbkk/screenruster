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

use std::thread;
use std::ops::Deref;
use channel::{self, Receiver, Sender, SendError};

use users;
use log::warn;

use crate::error;
use crate::config;
use super::Authenticate;

pub struct Auth {
	receiver: Receiver<Response>,
	sender:   Sender<Request>,
}

#[derive(Clone, Debug)]
pub enum Request {
	Authenticate(String),
}

#[derive(Clone, Debug)]
pub enum Response {
	Success,
	Failure,
}

impl Auth {
	pub fn spawn(config: config::Auth) -> error::Result<Auth> {
		let     user    = users::get_current_username().ok_or(error::Auth::UnknownUser)?;
		let mut methods = Vec::<Box<dyn Authenticate>>::new();

		#[cfg(feature = "auth-internal")]
		methods.push(Box::new(super::internal::new(config.get("internal"))?));

		#[cfg(feature = "auth-pam")]
		methods.push(Box::new(super::pam::new(config.get("pam"))?));

		let (sender, i_receiver) = channel::unbounded();
		let (i_sender, receiver) = channel::unbounded();

		thread::spawn(move || {
			'main: while let Ok(request) = receiver.recv() {
				match request {
					Request::Authenticate(password) => {
						if methods.is_empty() {
							warn!("no authentication method");

							sender.send(Response::Success).unwrap();
							continue 'main;
						}

						for method in &mut methods {
							if let Ok(true) = method.authenticate(user.to_str().unwrap(), &password) {
								sender.send(Response::Success).unwrap();
								continue 'main;
							}
						}

						sender.send(Response::Failure).unwrap();
					}
				}
			}
		});

		Ok(Auth {
			receiver: i_receiver,
			sender:   i_sender,
		})
	}

	pub fn authenticate<S: Into<String>>(&self, password: S) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Authenticate(password.into()))
	}
}

impl Deref for Auth {
	type Target = Receiver<Response>;

	fn deref(&self) -> &Receiver<Response> {
		&self.receiver
	}
}
