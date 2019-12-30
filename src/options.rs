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

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = "dau",
    about = "Do As User: Run commands as, or switch to, a user",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
pub struct Opts {
    /// Explain what is being done. Can be specified multiple times to
    /// increase logging.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u64,

    /// Execute as this user (default 'root')
    #[structopt(short = "u", long = "user")]
    pub user: Option<String>,

    /// Switch to a login shell for the specified user
    #[structopt(short = "i", long = "login")]
    pub login: bool,

    #[structopt()]
    pub command: Vec<String>,
}
