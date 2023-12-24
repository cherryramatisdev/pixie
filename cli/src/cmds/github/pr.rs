pub struct GithubPrCreateCmd {
    target_branch: String
}

impl GithubPrCreateCmd {
    pub fn new(target: &String) -> Self {
        Self {
            target_branch: target.to_string()
        }
    }

    pub fn call(&self) {
        println!("AQUI {:?}", self.target_branch);
    }
}
