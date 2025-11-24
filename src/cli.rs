// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use clap::{arg, command, crate_authors, Arg, Command};

pub fn build_cli() -> Command {
    command!()
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Shows all fortunes, including unkind."),
        )
        .arg(
            Arg::new("unkind")
                .short('o')
                .short('u')
                .long("unkind")
                .help("Shows only unkind fortunes."),
        )
        .arg(
            Arg::new("find")
                .short('m')
                .long("find")
                .value_name("pattern")
                .help("Finds fortunes matching regex query."),
        )
        .arg(
            Arg::new("length")
                .short('n')
                .long("length")
                .value_parser(clap::value_parser!(u8))
                .help("Finds a fortune that is shorter than provided number."),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .value_parser(clap::value_parser!(u8))
                .help("Sets a fixed width for the textbox. Defaults to 50."),
        )
        .arg(arg!(-s --short ... "Shows a short aporism."))
}
