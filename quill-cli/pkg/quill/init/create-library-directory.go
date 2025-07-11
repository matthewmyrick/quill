package quillInit

import (
	"fmt"
	"os"
)

func createLibraryDirectory(libraryPath string) (string, error) {
	err := os.MkdirAll(libraryPath, 0755)
	if err != nil {
		return "", fmt.Errorf("could not create library directory at %s: %w", libraryPath, err)
	}

	fmt.Println("Library directory created successfully at:", libraryPath)

	return libraryPath, nil
}
