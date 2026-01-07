use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "macnetmon",
    version,
    about = "Network interface bandwidth monitor"
)]
pub struct Args {
    #[arg(short, long)]
    pub interval: Option<u64>,

    #[arg(long)]
    pub hide_loopback: bool,

    #[arg(long)]
    pub hide_virtual: bool,

    #[arg(long)]
    pub show_inactive: bool,

    #[arg(long)]
    pub bits: bool,
}
