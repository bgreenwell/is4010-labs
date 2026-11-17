// Week 14: CLI application (capstone)
//
// Build a command-line password generator using the clap library.
//
// The logic lives in generator.rs and validator.rs.
// Your job here is to wire up the CLI: parse arguments and call the right functions.
//
// Run: cargo test           (tests both the library logic and CLI behavior)
//      cargo run -- --help  (see the CLI interface once implemented)
#![allow(unused_variables, unused_imports)]

mod generator;
mod validator;

use clap::{Parser, Subcommand};
use generator::{generate_passphrase, generate_pin, generate_random};
use validator::{calculate_entropy, check_common_patterns, validate_strength};

// ============================================================================
// CLI DEFINITION: complete as supplied; do not change these structs
// ============================================================================

/// A password generator CLI.
#[derive(Parser)]
#[command(name = "passgen", version, about = "Generate and validate passwords")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a random password.
    Random {
        /// Length of the password (default: 16).
        #[arg(short, long, default_value_t = 16)]
        length: usize,

        /// Include symbols such as !@#$%^&*.
        #[arg(short, long)]
        symbols: bool,
    },

    /// Generate a passphrase from random words.
    Passphrase {
        /// Number of words (default: 4).
        #[arg(short, long, default_value_t = 4)]
        words: usize,

        /// Separator character between words (default: '-').
        #[arg(short, long, default_value_t = '-')]
        separator: char,
    },

    /// Generate a numeric PIN.
    Pin {
        /// Length of the PIN (default: 6).
        #[arg(short, long, default_value_t = 6)]
        length: usize,
    },

    /// Check the strength of a password.
    Validate {
        /// The password to validate.
        password: String,
    },
}

// ============================================================================
// MAIN: implement the match arms below
// ============================================================================
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random { length, symbols } => {
            // TODO: call generate_random(length, symbols), then print
            //   Generated password: <password>
            //   Entropy: <bits> bits      (use calculate_entropy)
            todo!("Handle the `random` subcommand")
        }

        Commands::Passphrase { words, separator } => {
            // TODO: call generate_passphrase(words, separator), then print
            //   Generated passphrase: <passphrase>
            todo!("Handle the `passphrase` subcommand")
        }

        Commands::Pin { length } => {
            // TODO: call generate_pin(length), then print
            //   Generated PIN: <pin>
            todo!("Handle the `pin` subcommand")
        }

        Commands::Validate { password } => {
            // TODO: print
            //   Password strength: <strength>   (use validate_strength)
            //   Entropy: <bits> bits            (use calculate_entropy)
            // and, if check_common_patterns(&password) is true, a line starting with
            //   Warning:
            todo!("Handle the `validate` subcommand")
        }
    }
}
