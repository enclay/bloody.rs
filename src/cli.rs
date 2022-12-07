use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Action>,

    #[arg(short, long)]
    pub tray: bool,
}

#[derive(clap::Subcommand)]
pub enum Action {
   Level,
   SetLevel {
       level: u8,
   },
}
