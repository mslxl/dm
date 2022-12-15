use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct RootCmd {

}


fn main() {
    let args = RootCmd::parse();

}
