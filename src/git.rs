use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub fn execute_git_command(exec: &PathBuf, repo: &PathBuf, args: &Vec<String>) -> ExitStatus {
    let mut prog: Command = Command::new(exec.as_os_str());

    let cmd: &mut Command = prog
        .arg("-C")
        .arg(repo.as_os_str())
        .args(args);

    println!("> {:?}", cmd);
    println!(
        "> {:?} {:?}",
        // cmd.get_envs().collect(),
        cmd.get_program().to_str(),
        cmd.get_args().collect::<Vec<_>>(),
    );

    let status = cmd
        .status()
        .expect("failed to execute process");

    status
}
