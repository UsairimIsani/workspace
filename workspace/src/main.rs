use clap::Clap;

/// # Workspace
/// A Cli Tool to maintain your workspace
#[derive(Clap, Debug)]
#[clap(version = "0.0.1", author = "Usairim Isani <aunps@pm.me>")]
struct Opts {
    /// Sets a custom config file.
    #[clap(short, long)]
    config: Option<String>,

    /// A level of verbosity
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// A subcommand for setting up workspace
    #[clap(version = "0.0.1", author = "Usairim Isani <aunps@pm.me>")]
    Setup(Setup),
    /// A subcommand for initializing workspace
    #[clap(version = "0.0.1", author = "Usairim Isani <aunps@pm.me>")]
    Init(Init),
}

/// A subcommand for setting up workspace
#[derive(Clap, Debug)]
struct Setup {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}
/// A subcommand for initializing workspace
#[derive(Clap, Debug)]
struct Init {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("Value for config: {:?}", opts.config);

    // No Need of Verbosity yet.
    // match opts.verbose {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    match opts.sub_command {
        SubCommand::Init(init) => {
            // # TODO
            println!("Init {:?}", init)
        }
        SubCommand::Setup(setup) => {
            // # TODO
            println!("Setup {:?}", setup)
        }
    }
}
