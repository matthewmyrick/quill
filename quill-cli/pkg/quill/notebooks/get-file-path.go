package quillNotebooks

import (
	"fmt"
	"os"
)

// GetFilePath returns the path to the notebook file.
func GetFilePath(libraryPath string) (string, error) {
	notebookFilePath := fmt.Sprintf("%s/notebooks/quill.quill", libraryPath)
	if _, err := os.Stat(notebookFilePath); os.IsNotExist(err) {
		return "", fmt.Errorf("notebook file does not exist at %s", notebookFilePath)
	}
	return notebookFilePath, nil
}
