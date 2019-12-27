/*
 * Copyright © 2019, Steve Smith <tarkasteve@gmail.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version
 * 3 as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub use anyhow::{Result, Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DauError {
    #[error("The dau binary is not setuid root. See the dau documentation for installation instructions.")]
    NotSetUIDRoot,

    #[error("The config file has incorrect permissions; should be owned by root and not world readable or writable.")]
    InvalidConfigfilePermissions,

    #[error("Unsupported operation; this function should never be called on this OS.")]
    UnsupportedOperation,
}
