// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::io;

mod cli;
mod file;
mod fortune;
mod random;

fn main() -> io::Result<()> {
    let matches = cli::build_cli().get_matches();

    if let Some(pattern) = matches.get_one::<String>("find") {
        fortune::search_fortunes(pattern);
    } else if let Some(short) = matches.get_one::<u8>("short") {
        if let Some(width) = matches.get_one::<u8>("width") {
            fortune::get_quote(short, width);
        } else {
            fortune::get_quote(short, &50);
        }
    } else {
        if let Some(width) = matches.get_one::<u8>("width") {
            fortune::get_quote(&0, width);
        } else {
            fortune::get_quote(&0, &50);
        }
    }

    Ok(())
}
