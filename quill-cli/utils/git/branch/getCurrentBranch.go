package gitBranch

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// GetCurrentBranch reads the branch name from the .git/HEAD file.
func GetCurrentBranch(gitPath string) (string, error) {
	headFilePath := filepath.Join(gitPath, "HEAD")
	content, err := os.ReadFile(headFilePath)
	if err != nil {
		return "", fmt.Errorf("could not read HEAD file: %w", err)
	}

	contentStr := string(content)
	if strings.HasPrefix(contentStr, "ref: refs/heads/") {
		branch := strings.TrimSpace(strings.TrimPrefix(contentStr, "ref: refs/heads/"))
		return branch, nil
	}

	return "", fmt.Errorf("could not parse branch from HEAD file, are you in a detached HEAD state?")
}
