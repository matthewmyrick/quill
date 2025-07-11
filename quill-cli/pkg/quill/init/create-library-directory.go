package quillInit

import (
	"fmt"
	"os"
	"path/filepath"
)

func createLibraryDirectory(libraryPath string) (string, error) {
	// os.UserConfigDir() returns the default directory for user-specific
	// configuration files.
	// - Linux/macOS: ~/.config
	// - Windows: %APPDATA%
	if libraryPath == "" {
		libraryPath = defaultNotebookPath()
	}

	err := os.MkdirAll(libraryPath, 0755)
	if err != nil {
		return "", fmt.Errorf("could not create library directory at %s: %w", libraryPath, err)
	}

	fmt.Println("Library directory created successfully at:", libraryPath)

	return libraryPath, nil
}

func defaultNotebookPath() string {
	// os.UserConfigDir() returns the default directory for user-specific
	// configuration files.
	// - Linux/macOS: ~/.config
	// - Windows: %APPDATA%
	configDir, _ := os.UserConfigDir()
	return filepath.Join(configDir, "quill", "library")
}
