package quillInit

import (
	"fmt"
	"os"

	gitBranch "quill-cli/pkg/git/branch"
	gitInit "quill-cli/pkg/git/init"
	gitOrg "quill-cli/pkg/git/org"
	gitRepo "quill-cli/pkg/git/repo"
)

func createSkeleton(libraryPath string) (string, error) {
	gitRoot, err := gitInit.FindGitRoot()
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	localBranches, err := gitBranch.GetLocalBranches(gitRoot)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	orgName, err := gitOrg.GetOrgName(gitRoot)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	repoName, err := gitRepo.GetRepoName(gitRoot)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	if _, err := os.Stat(libraryPath + "/notebooks"); os.IsNotExist(err) {
		err = os.MkdirAll(libraryPath+"/notebooks", 0755)
		if err != nil {
			return "", fmt.Errorf("could not create notebooks directory at %s: %w", libraryPath+"/notebooks", err)
		}
	}

	// create a new notebook file for the organization/repository
	notebookFilePath := fmt.Sprintf("%s/%s/%s.md", libraryPath, orgName, repoName)
	err = os.MkdirAll(fmt.Sprintf("%s/%s", libraryPath, orgName), 0755)
	if err != nil {
		return "", fmt.Errorf("could not create directory for organization %s: %w", orgName, err)
	}
	file, err := os.Create(notebookFilePath)
	if err != nil {
		return "", fmt.Errorf("could not create notebook file %s: %w", notebookFilePath, err)
	}
	defer file.Close()
	_, err = file.WriteString(fmt.Sprintf("# Notebook: %s\n\n## Current Branch: %s\n\n## Available Local Branches:\n- %s\n", repoName, localBranches[0], localBranches))
	if err != nil {
		return "", fmt.Errorf("could not write to notebook file %s: %w", notebookFilePath, err)
	}
	return notebookFilePath, nil
}
