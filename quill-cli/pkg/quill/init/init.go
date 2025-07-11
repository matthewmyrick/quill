package quillInit

import (
	"fmt"

	quillHelpers "quill-cli/pkg/quill/helpers"
	quillNotebooks "quill-cli/pkg/quill/notebooks"
)

func Initialize(libraryPath string) (string, error) {
	if libraryPath == "" {
		libraryPath = quillHelpers.GetDefaultLibraryPath()
	}

	libraryPath, err := createLibraryDirectory(libraryPath)
	if err != nil {
		return "", fmt.Errorf("error initializing directory: %w", err)
	}

	_, err = quillNotebooks.CreateNotebookDir(libraryPath)
	if err != nil {
		return "", fmt.Errorf("error creating notebook directory: %w", err)
	}
	return libraryPath, nil
}
