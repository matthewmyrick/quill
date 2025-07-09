package cmd

import (
	"fmt"
	"os"

	gitBranch "quill-cli/pkg/git/branch"
	gitInit "quill-cli/pkg/git/init"
	gitRepo "quill-cli/pkg/git/repo"

	"github.com/spf13/cobra"
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initializes a new notebook based on the current git branch",
	Long:  `Traverses up the directory structure to find the .git repository, reads the current branch and repo name, and creates a new notebook file.`,
	Run: func(cmd *cobra.Command, args []string) {
		// This is the code that runs when `quill init` is executed.
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

		currentBranch, err := gitBranch.GetCurrentBranch(gitRoot)
		if err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}

		repoName, err := gitRepo.GetRepoName(gitRoot)
		if err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}

		fmt.Printf("Found repository: %s\n", repoName)
		fmt.Printf("Current git branch is: %s\n", currentBranch)
		fmt.Printf("Available local branches: %v\n", localBranches)
		fmt.Printf("Initializing notebook 'Notebook: %s'...\n", repoName)
	},
}
