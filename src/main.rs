use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Rusty Toolbox")]
#[command(version = "1.0")]
#[command(author = "Eric Zimmerman <ericjzim@gmail.com>")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// reformat input
    Reformat {
        /// Required argument for get
        #[arg(required = true)]
        raw_input: String,

        /// Sets the output format
        #[arg(short = 'o', long = "outputformat")]
        outputformat: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {} // Do nothing for case 0
        1 => println!("Debug level 1"),
        2 => println!("Debug level 2"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Reformat {
            raw_input,
            outputformat,
        }) => {
            let desired_format = outputformat.as_deref().unwrap_or("");
            reformat_input(raw_input, desired_format);
        }
        None => {}
    }

    // Continued program logic goes here...
}

fn reformat_input(input: &str, output_format: &str) {
    match output_format {
        "" => {
            // Get the input hex string from arguments and trim whitespace
            let input = input.trim();

            // Convert the hex string to a C array and print the result
            match hex_string_to_c_array(input) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Error: {}", e),
            }
        }
        "bin" => {
            let bin = input
                .as_bytes()
                .iter()
                .map(|b| format!("{:08b}", b))
                .collect::<String>();
            println!("Binary: {}", bin);
        }
        _ => eprintln!("Invalid output format"),
    }
}

fn hex_string_to_c_array(input: &str) -> Result<String, &'static str> {
    // Normalize input by removing spaces and `0x` prefixes
    let normalized_input = input.replace(" ", "").replace("0x", "");

    // Ensure the input length is even for valid byte pairs
    if normalized_input.len() % 2 != 0 {
        return Err("Invalid input: Length of input string is not even.");
    }

    // Split the normalized input into chunks of two characters
    // and convert each chunk into a formatted string with `0x` prefix
    let hex_bytes: Vec<String> = normalized_input
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            // Convert each chunk to a slice and then to a string
            let hex_str = std::str::from_utf8(chunk).unwrap(); // In real application, handle this Result properly
                                                               // Format the string with `0x` prefix
            format!("0x{}", hex_str.to_uppercase())
        })
        .collect();

    // Join the formatted strings with `, ` and wrap with `{ }` for C-style initialization
    Ok(format!("{{ {} }}", hex_bytes.join(", ")))
}
