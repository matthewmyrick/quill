package gitInit

import (
	"fmt"
	"os"
	"path/filepath"
)

// FindGitRoot traverses up from the current directory to find the .git directory.
func FindGitRoot() (string, error) {
	currentDir, err := os.Getwd()
	if err != nil {
		return "", err
	}

	for {
		gitPath := filepath.Join(currentDir, ".git")
		info, err := os.Stat(gitPath)
		if err == nil && info.IsDir() {
			return gitPath, nil
		}
		parentDir := filepath.Dir(currentDir)
		if parentDir == currentDir {
			break
		}
		currentDir = parentDir
	}

	return "", fmt.Errorf("not a git repository (or any of the parent directories)")
}
