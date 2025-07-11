package gitOrg

import (
	"fmt"
	"strings"

	gitInit "quill-cli/pkg/git/init"
	gitMeta "quill-cli/pkg/git/meta"
)

// GetOrgName parses the organization name from the .git/config file.
func GetOrgName() (string, error) {
	gitPath, err := gitInit.FindGitRoot()
	if err != nil {
		return "", err
	}
	url, err := gitMeta.GetRemoteURL(gitPath)
	if err != nil {
		return "", err
	}

	var path string
	if strings.HasPrefix(url, "https://") {
		path = strings.TrimPrefix(url, "https://")
		path = strings.TrimPrefix(path, "github.com/")
	} else if strings.HasPrefix(url, "git@") {
		path = strings.TrimPrefix(url, "git@github.com:")
	} else {
		return "", fmt.Errorf("unsupported git url format: %s", url)
	}

	parts := strings.Split(path, "/")
	if len(parts) < 2 {
		return "", fmt.Errorf("could not parse organization from path: %s", path)
	}

	return parts[0], nil
}
