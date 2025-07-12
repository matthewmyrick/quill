package quillPages

import (
	"fmt"
	"os"

	gitBranch "quill-cli/pkg/git/branch"
	gitOrg "quill-cli/pkg/git/org"
	gitRepo "quill-cli/pkg/git/repo"

	quillPageTypes "quill-cli/pkg/quill/pages/types"
)

func CreatePageDir(notebookPath string) (quillPageTypes.Page, error) {
	if notebookPath == "" {
		return quillPageTypes.Page{}, fmt.Errorf("notebook path cannot be empty")
	}
	if _, err := os.Stat(notebookPath); os.IsNotExist(err) {
		return quillPageTypes.Page{}, fmt.Errorf("notebook path does not exist: %s", notebookPath)
	}

	pagesDir := notebookPath + "/pages"

	//check if pages directory exists, if not create it
	if _, err := os.Stat(pagesDir); os.IsNotExist(err) {
		err = os.MkdirAll(pagesDir, 0755)
		if err != nil {
			return quillPageTypes.Page{}, fmt.Errorf("could not create pages directory at %s: %w", pagesDir, err)
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

	pageName := fmt.Sprintf("%s-%s-%s", orgName, repoName, branch)
	pageDir := fmt.Sprintf("%s/%s", pagesDir, pageName)

	// create the page data structure
	if _, err := os.Stat(pageDir); os.IsNotExist(err) {
		err = os.MkdirAll(pageDir, 0755)
		if err != nil {
			return quillPageTypes.Page{}, fmt.Errorf("could not create pages directory at %s: %w", pageDir, err)
		}
	}

	pageData := quillPageTypes.PageData{
		Name:        pageName,
		Branch:      branch,
		Repo:        repoName,
		Org:         orgName,
		PagePath:    pageDir,
		Description: "A page for the " + repoName + " repository.",
		Tags:        []string{"git-repo", "auto-initialized"},
	}

	pg, err := CreatePage(pageData)
	if err != nil {
		return quillPageTypes.Page{}, fmt.Errorf("could not create page: %w", err)
	}

	return *pg, nil
}
