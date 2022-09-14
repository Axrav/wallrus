pub mod argparse;
pub mod functions;
mod set;

fn main() {
    let args = argparse::arguments().get_matches();

    match args.subcommand() {
        Some(("set", subc)) => {
            set::set_wallpaper(subc).unwrap();
        }
        _ => println!("Lost?? try --help"),
    }
}
