pub mod pr {
    use std::process::{Command, Stdio};

    pub struct Pr {
        pub target_branch: String,
        pub reviewers: Vec<String>,
        pub is_draft: bool,
    }

    impl Pr {
        pub fn create(self) -> Result<Self, std::io::Error> {
            let mut gh_pr = Command::new("gh");
            let gh_pr = gh_pr.stdin(Stdio::inherit()).stdout(Stdio::inherit());
            let gh_pr = gh_pr
                .args(&["pr", "create"])
                .args(&["--base", self.target_branch.as_str()])
                .args(&["--assignee", "@me"]);

            if self.is_draft {
                gh_pr.arg("--draft");
            }

            if self.reviewers.len() > 0 {
                gh_pr.args(&["--reviewer", &self.reviewers.join(",")]);
            }

            gh_pr.status()?;

            return Ok(self);
        }

        pub fn open(self) -> Self {
            eprintln!("OPEN");
            return self;
        }
    }
}
