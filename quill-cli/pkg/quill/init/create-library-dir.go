package quillInit

import (
	"fmt"
	"os"
)

func createLibraryDir(libraryPath string) (string, error) {
	if libraryPath == "" {
		return "", fmt.Errorf("library path cannot be empty")
	}
	if _, err := os.Stat(libraryPath); !os.IsNotExist(err) {
		fmt.Println("Library directory already exists at:", libraryPath)
		return libraryPath, nil
	}
	err := os.MkdirAll(libraryPath, 0755)
	if err != nil {
		return "", fmt.Errorf("could not create library directory at %s: %w", libraryPath, err)
	}

	fmt.Println("Library directory created successfully at:", libraryPath)

	return libraryPath, nil
}
