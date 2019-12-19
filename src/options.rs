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

use structopt::StructOpt;

use crate::errors::Result;


#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = "xcp",
    about = "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
pub struct Opts {
    /// Explain what is being done. Can be specified multiple times to
    /// increase logging.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u64,

}
