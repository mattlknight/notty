//  notty is a new kind of terminal emulator.
//  Copyright (C) 2015 without boats
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub scrollback: u32,
    pub tab_stop: u32,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            scrollback: 512,
            tab_stop: 4,
        }
    }
}
