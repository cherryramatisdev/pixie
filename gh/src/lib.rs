pub mod pr {
    use std::process::{Command, Stdio};

    pub struct Pr {
        pub target_branch: String,
        pub reviewers: Vec<String>,
        pub is_draft: bool,
    }

    impl Pr {
        pub fn create(self) -> Result<Self, std::io::Error> {
            let mut cmd = Command::new("gh");
            let cmd = cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit());
            let cmd = cmd
                .args(["pr", "create"])
                .args(["--base", self.target_branch.as_str()])
                .args(["--assignee", "@me"]);

            if self.is_draft {
                cmd.arg("--draft");
            }

            if !self.reviewers.is_empty() {
                cmd.args(["--reviewer", &self.reviewers.join(",")]);
            }

            cmd.status()?;

            Ok(self)
        }

        pub fn open(self) -> Result<Self, std::io::Error> {
            let mut cmd = Command::new("gh");
            let cmd = cmd.args(["pr", "view", "-w"]);

            cmd.status()?;

            Ok(self)
        }
    }
}

pub mod profile {
    use std::{
        env,
        fs::OpenOptions,
        io::Write,
        path::PathBuf,
        process::Command,
    };

    pub fn switch() -> anyhow::Result<()> {
        Command::new("gh").args(["auth", "switch"]).output()?;

        let home_dir = env::var("HOME").expect("HOME directory doesn't exist");
        let cache_target_path = PathBuf::from(format!("{}/.local/share/gh_profile.txt", home_dir));

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(cache_target_path)?;

        let output = Command::new("gh").args(["auth", "status"]).output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Couldn't run the github auth status command"
            ));
        }

        let output = String::from_utf8_lossy(&output.stdout).to_string();

        write!(file, "{}", output)?;

        Ok(())
    }
}
