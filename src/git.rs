use anyhow::{anyhow, Result};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitContext {
    pub org: String,
    pub repo: String,
    pub branch: String,
}

impl GitContext {
    pub fn from_current_dir() -> Result<Self> {
        match Repository::discover(".") {
            Ok(repo) => {
                let workdir = repo.workdir().ok_or_else(|| anyhow!("Not in a git repository"))?;
                
                let repo_name = Self::extract_repo_name(workdir)?;
                let org_name = Self::extract_org_name(&repo).unwrap_or_else(|_| "local".to_string());
                let branch_name = Self::get_current_branch(&repo).unwrap_or_else(|_| "main".to_string());

                Ok(GitContext {
                    org: org_name,
                    repo: repo_name,
                    branch: branch_name,
                })
            }
            Err(_) => {
                // Not in a git repository, create a default context
                let current_dir = std::env::current_dir()?;
                let dir_name = current_dir
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("quill-tasks")
                    .to_string();
                
                Ok(GitContext {
                    org: "local".to_string(),
                    repo: dir_name,
                    branch: "default".to_string(),
                })
            }
        }
    }

    fn extract_repo_name(workdir: &Path) -> Result<String> {
        workdir
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Could not extract repository name"))
    }

    fn extract_org_name(repo: &Repository) -> Result<String> {
        let config = repo.config()?;
        let remote_url = config
            .get_string("remote.origin.url")
            .or_else(|_| {
                // Try to get the first remote if origin doesn't exist
                let remotes = repo.remotes()?;
                if let Some(remote_name) = remotes.get(0) {
                    config.get_string(&format!("remote.{}.url", remote_name))
                } else {
                    Err(git2::Error::from_str("No remotes found"))
                }
            })
            .unwrap_or_else(|_| "local".to_string());

        // Extract org from various URL formats
        if let Some(org) = Self::parse_org_from_url(&remote_url) {
            Ok(org)
        } else {
            Ok("local".to_string())
        }
    }

    fn parse_org_from_url(url: &str) -> Option<String> {
        // Handle GitHub SSH URLs: git@github.com:org/repo.git
        if url.starts_with("git@github.com:") {
            return url
                .strip_prefix("git@github.com:")?
                .split('/')
                .next()
                .map(|s| s.to_string());
        }

        // Handle HTTPS URLs: https://github.com/org/repo.git
        if url.starts_with("https://github.com/") {
            return url
                .strip_prefix("https://github.com/")?
                .split('/')
                .next()
                .map(|s| s.to_string());
        }

        // Handle other Git hosting services similarly
        if let Some(domain_start) = url.find("://") {
            let after_protocol = &url[domain_start + 3..];
            if let Some(path_start) = after_protocol.find('/') {
                let path = &after_protocol[path_start + 1..];
                return path.split('/').next().map(|s| s.to_string());
            }
        }

        None
    }

    fn get_current_branch(repo: &Repository) -> Result<String> {
        let head = repo.head()?;
        if let Some(branch_name) = head.shorthand() {
            Ok(branch_name.to_string())
        } else {
            // Handle detached HEAD state
            let oid = head.target().ok_or_else(|| anyhow!("HEAD has no target"))?;
            Ok(format!("detached-{}", &oid.to_string()[..8]))
        }
    }

    pub fn context_key(&self) -> String {
        format!("{}:{}:{}", self.org, self.repo, self.branch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_context_creation() {
        let context = GitContext {
            org: "testorg".to_string(),
            repo: "testrepo".to_string(),
            branch: "main".to_string(),
        };
        
        assert_eq!(context.org, "testorg");
        assert_eq!(context.repo, "testrepo");
        assert_eq!(context.branch, "main");
    }

    #[test]
    fn test_context_key() {
        let context = GitContext {
            org: "myorg".to_string(),
            repo: "myrepo".to_string(),
            branch: "feature".to_string(),
        };
        
        assert_eq!(context.context_key(), "myorg:myrepo:feature");
    }

    #[test]
    fn test_parse_github_ssh_url() {
        let url = "git@github.com:octocat/Hello-World.git";
        let org = GitContext::parse_org_from_url(url);
        assert_eq!(org, Some("octocat".to_string()));
    }

    #[test]
    fn test_parse_github_https_url() {
        let url = "https://github.com/octocat/Hello-World.git";
        let org = GitContext::parse_org_from_url(url);
        assert_eq!(org, Some("octocat".to_string()));
    }

    #[test]
    fn test_parse_invalid_url() {
        let url = "not-a-git-url";
        let org = GitContext::parse_org_from_url(url);
        assert_eq!(org, None);
    }

    #[test]
    fn test_git_context_serialization() {
        let context = GitContext {
            org: "testorg".to_string(),
            repo: "testrepo".to_string(),
            branch: "main".to_string(),
        };
        
        let json = serde_json::to_string(&context).unwrap();
        let deserialized: GitContext = serde_json::from_str(&json).unwrap();
        
        assert_eq!(context, deserialized);
    }

    #[test]
    fn test_from_current_dir_fallback() {
        let context = GitContext::from_current_dir().unwrap();
        
        assert!(!context.org.is_empty());
        assert!(!context.repo.is_empty());
        assert!(!context.branch.is_empty());
    }
}