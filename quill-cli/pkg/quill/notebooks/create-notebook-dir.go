package quillNotebooks

import (
	"fmt"
	"os"

	gitOrg "quill-cli/pkg/git/org"
	gitRepo "quill-cli/pkg/git/repo"

	quillNotebooksTypes "quill-cli/pkg/quill/notebooks/types"
)

func CreateNotebookDir(libraryPath string) (quillNotebooksTypes.NotebookData, error) {
	if libraryPath == "" {
		return quillNotebooksTypes.NotebookData{}, fmt.Errorf("library path cannot be empty")
	}
	if _, err := os.Stat(libraryPath); os.IsNotExist(err) {
		return quillNotebooksTypes.NotebookData{}, fmt.Errorf("library path does not exist: %s", libraryPath)
	}

	notebooksDir := libraryPath + "/notebooks"
	if _, err := os.Stat(notebooksDir); os.IsNotExist(err) {
		err = os.MkdirAll(notebooksDir, 0755)
		if err != nil {
			return quillNotebooksTypes.NotebookData{}, fmt.Errorf("could not create notebooks directory at %s: %w", notebooksDir, err)
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

	notebookDirName := fmt.Sprintf("%s-%s", orgName, repoName)
	notebookDir := fmt.Sprintf("%s/%s", notebooksDir, notebookDirName)

	notebookData := quillNotebooksTypes.NotebookData{
		Name:         notebookDirName,
		Repo:         repoName,
		Org:          orgName,
		NotebookPath: notebookDir,
		Description:  "A notebook for the " + repoName + " repository.",
		Tags:         []string{"git-repo", "auto-initialized"},
	}

	// Check if the notebook directory already exists
	if _, err := os.Stat(notebookDir); !os.IsNotExist(err) {
		fmt.Println("Notebook directory already exists at:", notebookDir)
		return notebookData, nil
	}
	// create the notebook data structure
	if _, err := os.Stat(notebookDir); os.IsNotExist(err) {
		err = os.MkdirAll(notebookDir, 0755)
		if err != nil {
			return quillNotebooksTypes.NotebookData{}, fmt.Errorf("could not create notebooks directory at %s: %w", notebookDir, err)
		}
	}
	fmt.Println("Notebook directory created successfully at:", notebookDir)

	return notebookData, nil

}
