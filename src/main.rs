use std::env;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};

use nix::mount::{mount, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{chdir, chroot, sethostname};

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let args = args.iter().map(String::as_str).collect::<Vec<&str>>();

    match &args[1..] {
        ["run", cmd, args @ ..] => {
            let mut child = run(cmd, args)?;
            let exit_code = child.wait()?;
            println!("Command finished running with code {}", exit_code);
        }

        _ => println!("./container run <command> [<args>...]"),
    };

    Ok(())
}

fn process_setup() -> std::io::Result<()> {
    unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID)?;
    sethostname("container")?;
    chroot("/home/dario/images/ubuntu-18.04-minimal-cloudimg-amd64-root")?;
    chdir("/")?;
    mount::<str, str, str, str>(Some("proc"), "proc", Some("proc"), MsFlags::empty(), None)?;

    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> std::io::Result<Child> {
    println!("Running: {} {}", cmd, args.join(" "));
    unsafe { Command::new(cmd).args(args).pre_exec(process_setup).spawn() }
}
