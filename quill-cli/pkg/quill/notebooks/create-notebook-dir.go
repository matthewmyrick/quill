package quillNotebooks

import (
	"fmt"
	"os"

	gitBranch "quill-cli/pkg/git/branch"
	gitOrg "quill-cli/pkg/git/org"
	gitRepo "quill-cli/pkg/git/repo"

	quillNotebooksTypes "quill-cli/pkg/quill/notebooks/types"
)

func CreateNotebookDir(libraryPath string) (quillNotebooksTypes.Notebook, error) {
	if libraryPath == "" {
		return quillNotebooksTypes.Notebook{}, fmt.Errorf("library path cannot be empty")
	}
	if _, err := os.Stat(libraryPath); os.IsNotExist(err) {
		return quillNotebooksTypes.Notebook{}, fmt.Errorf("library path does not exist: %s", libraryPath)
	}

	notebookDir := libraryPath + "/notebooks"
	if _, err := os.Stat(notebookDir); os.IsNotExist(err) {
		err = os.MkdirAll(notebookDir, 0755)
		if err != nil {
			return quillNotebooksTypes.Notebook{}, fmt.Errorf("could not create notebooks directory at %s: %w", notebookDir, err)
		}
	}

	orgName, err := gitOrg.GetOrgName()
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	repoName, err := gitRepo.GetRepoName()
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	branch, err := gitBranch.GetCurrentBranch()
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	notebookName := fmt.Sprintf("%s-%s", orgName, repoName)

	notebookData := quillNotebooksTypes.NotebookData{
		Name:         notebookName,
		Branch:       branch,
		Repo:         repoName,
		Org:          orgName,
		NotebookPath: notebookDir,
		Description:  "A notebook for the " + repoName + " repository.",
		Tags:         []string{"git-repo", "auto-initialized"},
	}

	nb, err := createNotebook(notebookData)
	if err != nil {
		return quillNotebooksTypes.Notebook{}, fmt.Errorf("could not create notebook: %w", err)
	}

	return *nb, nil
}
