package gitRepo

import (
	"fmt"
	"strings"

	gitInit "quill-cli/pkg/git/init"
	gitMeta "quill-cli/pkg/git/meta"
)

// GetRepoName parses the repository name from the .git/config file.
func GetRepoName() (string, error) {
	gitPath, err := gitInit.FindGitRoot()
	if err != nil {
		return "", err
	}
	url, err := gitMeta.GetRemoteURL(gitPath)
	if err != nil {
		return "", err
	}

	// Handles both https://github.com/org/repo.git and git@github.com:org/repo.git
	lastSlash := strings.LastIndex(url, "/")
	lastColon := strings.LastIndex(url, ":")
	lastSeparator := lastSlash
	if lastColon > lastSlash {
		lastSeparator = lastColon
	}
	if lastSeparator == -1 {
		return "", fmt.Errorf("could not parse repo name from url: %s", url)
	}

	repoPart := url[lastSeparator+1:]
	repoName := strings.TrimSuffix(repoPart, ".git")
	return repoName, nil
}
