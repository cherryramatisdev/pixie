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
                .args(&["pr", "create"])
                .args(&["--base", self.target_branch.as_str()])
                .args(&["--assignee", "@me"]);

            if self.is_draft {
                cmd.arg("--draft");
            }

            if self.reviewers.len() > 0 {
                cmd.args(&["--reviewer", &self.reviewers.join(",")]);
            }

            cmd.status()?;

            return Ok(self);
        }

        pub fn open(self) -> Result<Self, std::io::Error> {
            let mut cmd = Command::new("gh");
            let cmd = cmd.args(&["pr", "view", "-w"]);

            cmd.status()?;

            return Ok(self);
        }
    }
}
