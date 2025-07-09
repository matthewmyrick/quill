package cmd

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initializes a new notebook based on the current git branch",
	Long:  `Traverses up the directory structure to find the .git repository, reads the current branch and repo name, and creates a new notebook file.`,
	Run: func(cmd *cobra.Command, args []string) {
		// This is the code that runs when `quill init` is executed.
		gitRoot, err := findGitRoot()
		if err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}

		branch, err := getCurrentBranch(gitRoot)
		if err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}

		repoName, err := getRepoName(gitRoot)
		if err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}

		fmt.Printf("Found repository: %s\n", repoName)
		fmt.Printf("Current git branch is: %s\n", branch)
		fmt.Printf("Initializing notebook 'Notebook: %s'...\n", repoName)

		// Here you would add your logic to create and write the initial .quill file
		// For example:
		// fileName := filepath.Join(filepath.Dir(gitRoot), repoName+".quill")
		// content := []byte(fmt.Sprintf("Notebook: %s\n", repoName))
		// os.WriteFile(fileName, content, 0644)
	},
}

// findGitRoot traverses up from the current directory to find the .git directory.
func findGitRoot() (string, error) {
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

		// Move up to the parent directory
		parentDir := filepath.Dir(currentDir)
		if parentDir == currentDir {
			// Reached the root of the filesystem
			break
		}
		currentDir = parentDir
	}

	return "", fmt.Errorf("not a git repository (or any of the parent directories)")
}

// getCurrentBranch reads the branch name from the .git/HEAD file.
func getCurrentBranch(gitPath string) (string, error) {
	headFilePath := filepath.Join(gitPath, "HEAD")
	content, err := os.ReadFile(headFilePath)
	if err != nil {
		return "", fmt.Errorf("could not read HEAD file: %w", err)
	}

	// The content of HEAD is typically "ref: refs/heads/main"
	contentStr := string(content)
	if strings.HasPrefix(contentStr, "ref: refs/heads/") {
		branch := strings.TrimSpace(strings.TrimPrefix(contentStr, "ref: refs/heads/"))
		return branch, nil
	}

	return "", fmt.Errorf("could not parse branch from HEAD file, are you in a detached HEAD state?")
}

// getRepoName parses the repository name from the .git/config file.
func getRepoName(gitPath string) (string, error) {
	configFilePath := filepath.Join(gitPath, "config")
	file, err := os.Open(configFilePath)
	if err != nil {
		return "", fmt.Errorf("could not open git config file: %w", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	inRemoteSection := false
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "[remote \"origin\"]" {
			inRemoteSection = true
			continue
		}
		if inRemoteSection && strings.HasPrefix(line, "url =") {
			// Line is like: url = git@github.com:your-username/quill.git
			parts := strings.Split(line, "=")
			if len(parts) < 2 {
				continue
			}
			url := strings.TrimSpace(parts[1])

			// Get the part after the last / or :
			lastSlash := strings.LastIndex(url, "/")
			lastColon := strings.LastIndex(url, ":")

			lastSeparator := lastSlash
			if lastColon > lastSlash {
				lastSeparator = lastColon
			}

			if lastSeparator == -1 {
				continue // Should not happen for valid git URLs
			}

			repoPart := url[lastSeparator+1:]
			repoName := strings.TrimSuffix(repoPart, ".git")
			return repoName, nil
		}
		// If we encounter another section header, we're done with the remote section
		if inRemoteSection && strings.HasPrefix(line, "[") {
			break
		}
	}

	if err := scanner.Err(); err != nil {
		return "", fmt.Errorf("error scanning git config file: %w", err)
	}

	return "", fmt.Errorf("could not find remote origin url in git config")
}

func init() {
	// This function is called when the application starts.
	// We add the `initCmd` to our `rootCmd` here.
	rootCmd.AddCommand(initCmd)
}
