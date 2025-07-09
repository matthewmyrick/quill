package quillInit

import (
	"fmt"
	"os"
	"path/filepath"
)

func createLibraryDirectory() (string, error) {
	// os.UserConfigDir() returns the default directory for user-specific
	// configuration files.
	// - Linux/macOS: ~/.config
	// - Windows: %APPDATA%
	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", fmt.Errorf("could not get user config directory: %w", err)
	}

	libraryPath := filepath.Join(configDir, "quill", "library")

	err = os.MkdirAll(libraryPath, 0755)
	if err != nil {
		return "", fmt.Errorf("could not create library directory at %s: %w", libraryPath, err)
	}

	return libraryPath, nil
}
