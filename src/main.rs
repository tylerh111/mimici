mod app;
mod cli;
mod error;
mod git;
mod init;
mod manifest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
