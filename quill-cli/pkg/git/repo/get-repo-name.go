package gitRepo

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// GetRepoName parses the repository name from the .git/config file.
func GetRepoName(gitPath string) (string, error) {
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
			parts := strings.Split(line, "=")
			if len(parts) < 2 {
				continue
			}
			url := strings.TrimSpace(parts[1])

			lastSlash := strings.LastIndex(url, "/")
			lastColon := strings.LastIndex(url, ":")

			lastSeparator := lastSlash
			if lastColon > lastSlash {
				lastSeparator = lastColon
			}

			if lastSeparator == -1 {
				continue
			}

			repoPart := url[lastSeparator+1:]
			repoName := strings.TrimSuffix(repoPart, ".git")
			return repoName, nil
		}
		if inRemoteSection && strings.HasPrefix(line, "[") {
			break
		}
	}

	if err := scanner.Err(); err != nil {
		return "", fmt.Errorf("error scanning git config file: %w", err)
	}

	return "", fmt.Errorf("could not find remote origin url in git config")
}
