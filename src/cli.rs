use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ArgsTest {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("test"))]
    pub name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

}


