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
