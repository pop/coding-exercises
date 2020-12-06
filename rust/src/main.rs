use clap::{crate_version, Clap};

mod hanoi;

#[derive(Clap)]
#[clap(version = crate_version!(), author = "Elijah Voigt <elijah.caine.mv@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    exercise: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.1", about = "Towers of Hanoi")]
    Hanoi,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.exercise {
        SubCommand::Hanoi => {
            hanoi::run();
        }
    }
}