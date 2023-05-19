/*
   Copyright 2023 Marek Suchánek

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::fs::File;

use color_eyre::eyre::{Result, WrapErr};

mod cli;
mod report;
mod vale;
mod logging;

fn main() -> Result<()> {
    // Enable full-featured error logging.
    color_eyre::install()?;

    // Load command-line arguments.
    let args = cli::arguments();

    // Configure logging based on the set verbosity level.
    logging::initialize_logger(args.verbose)?;

    let json = match args.variant {
        cli::Variants::Input { input } => input,
        cli::Variants::File { file } => {
            std::fs::read_to_string(file)
                .wrap_err("Failed to read the input file.")?
        }
    };

    let deserialized: vale::Alerts = serde_json::from_str(&json)
        .wrap_err("Failed to parse the input file.")?;
    //println!("{:#?}", deserialized);

    let report = report::junit_report(deserialized);

    let mut file = File::create(args.out)
        .wrap_err("Failed to create the output file.")?;
    report.write_xml(&mut file)
        .wrap_err("Failed to write to the output file.")?;

    Ok(())
}
