use clap::Parser;


#[derive(Parser)]
#[clap(name = "git")]
#[clap(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
#[clap(name = "rfood")]
#[clap(bin_name = "rfood")]
pub enum Command {
    #[clap(arg_required_else_help = true)]
    /// Basic CLI to run parkrun queries 
    Run {
        /// Parkrun athete ID e.g. A123456
        #[arg(short, long)]
        id: String,

        /// Parkrun password 
        #[arg(short, long)]
        password: String,
    },
}
