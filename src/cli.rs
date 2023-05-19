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

use std::path::PathBuf;

use bpaf::Bpaf;

/// Define the command-line arguments of the tool.
#[must_use]
pub fn arguments() -> Cli {
    let usage_prefix = "Usage: vale2junit {usage}";
    cli().usage(usage_prefix).run()
}

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
/// Convert the JSON output from Vale to the JUnit format.
pub struct Cli {
    /// Display more detailed progress messages.
    #[bpaf(short, long, switch, many, map(vec_len))]
    pub verbose: usize,

    #[bpaf(external(variants))]
    pub variant: Variants,
}

#[derive(Clone, Debug, Bpaf)]
pub enum Variants {
    File {
        /// Path to the JSON file.
        #[bpaf(short, long, argument("FILE"))]
        file: PathBuf,
    },
    Input {
        /// The JSON string passed on the command line.
        #[bpaf(short, long, argument("JSON"))]
        input: String,
    },
}

/// Calculate the length of a vector for repeating flags, such as verbosity.
///
/// This function has to take the argument by value because that's how
/// the `bpaf` parser passes it in the map application.
#[allow(clippy::needless_pass_by_value)]
fn vec_len<T>(vec: Vec<T>) -> usize {
    vec.len()
}
