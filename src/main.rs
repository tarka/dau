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

mod errors;
mod options;

use log::info;
use simplelog::{Config, LevelFilter, SimpleLogger, TermLogger, TerminalMode};
use structopt::StructOpt;

use crate::errors::{Result, DauError};

fn main() -> Result<()> {
    let opts = options::Opts::from_args();

    let log_level = match opts.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    TermLogger::init(log_level, Config::default(), TerminalMode::Mixed)
        .or_else(|_| SimpleLogger::init(log_level, Config::default()))?;

    Ok(())
}
