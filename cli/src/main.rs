use clap::{value_parser, Arg, ArgAction, Command};
use clap_complete::{generate, Generator, Shell};

fn cli() -> Command {
    let gh_pr_commands = Command::new("pull-request")
        .alias("pr")
        .subcommands([Command::new("create").arg(Arg::new("target_branch").required(true))]);

    let gh_commands = Command::new("github")
        .about("Github operations")
        .alias("gh")
        .subcommands([gh_pr_commands]);

    return Command::new("pixie")
        .bin_name("pixie")
        .subcommands([gh_commands])
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        );
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout())
}

fn main() {
    let matches = cli().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }

    if let Some(commands) = matches.subcommand() {
        match commands {
            ("github", gh_commands) => {
                if let Some(("pull-request", pr_commands)) = gh_commands.subcommand() {
                    match pr_commands.subcommand() {
                        Some(("create", create_cmd)) => {
                            let target_branch = create_cmd
                                .get_one::<String>("target_branch")
                                .expect("Target branch is required");

                            cmds::github::pr::GithubPrCreateCmd::new(target_branch).call();

                            return;
                        }
                        _ => {
                            println!("Invalid command under 'pull-request'");
                            return;
                        }
                    }
                }
            }
            _ => {
                println!("Invalid subcommand");
                return;
            }
        }
    }
}
