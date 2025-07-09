package gitBranch

import (
	"fmt"
	"os"
	"path/filepath"
)

// GetLocalBranches retrieves all local branches from the git repository.
func GetLocalBranches(gitPath string) ([]string, error) {
	branches := []string{}
	branchesPath := filepath.Join(gitPath, "refs", "heads")
	files, err := os.ReadDir(branchesPath)
	if err != nil {
		return nil, fmt.Errorf("could not read branches directory: %w", err)
	}

	for _, file := range files {
		if !file.IsDir() {
			branches = append(branches, file.Name())
		}
	}

	return branches, nil
}
