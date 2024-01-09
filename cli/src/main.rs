use clap::{value_parser, Arg, ArgAction, Command};
use clap_complete::{generate, Generator, Shell};

fn cli() -> Command {
    let gh_pr_commands = Command::new("pull-request")
        .alias("pr")
        .subcommands([Command::new("create").arg(Arg::new("target_branch").required(true))]);

    let gh_profile_commands = Command::new("profile")
        .alias("p")
        .subcommands([Command::new("switch").alias("s")]);

    let gh_commands = Command::new("github")
        .about("Github operations")
        .alias("gh")
        .subcommands([gh_pr_commands, gh_profile_commands]);

    Command::new("pixie")
        .bin_name("pixie")
        .subcommands([gh_commands])
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout())
}

fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }

    // TODO: I don't like how all the Err() are staggered up
    // Probably the only actual usefulness of exceptions? (joking)
    return match matches.subcommand().unwrap() {
        ("github", commands) => match commands.subcommand().unwrap() {
            ("pull-request", pr_cmd) => match pr_cmd.subcommand().unwrap() {
                ("create", create_cmd) => {
                    let target_branch = create_cmd
                        .get_one::<String>("target_branch")
                        .expect("Target branch is required");

                    let pr = gh::pr::Pr {
                        target_branch: target_branch.to_string(),
                        reviewers: vec![],
                        is_draft: true,
                    };

                    pr.create()?.open()?;

                    Ok(())
                }
                _ => Err(anyhow::anyhow!("Error while matching subcommand")),
            },
            ("profile", profile_cmd) => match profile_cmd.subcommand().unwrap() {
                ("switch", _) => {
                    gh::profile::switch()?;

                    Ok(())
                }
                _ => Err(anyhow::anyhow!("Error while matching subcommand")),
            },
            _ => Err(anyhow::anyhow!("Error while matching subcommand")),
        },
        _ => Err(anyhow::anyhow!("Error while matching subcommand")),
    };
}
