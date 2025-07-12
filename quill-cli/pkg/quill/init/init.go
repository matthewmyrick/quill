package quillInit

import (
	"fmt"

	quillHelpers "quill-cli/pkg/quill/helpers"
	quillNotebooks "quill-cli/pkg/quill/notebooks"
	quillPages "quill-cli/pkg/quill/pages"
)

func Initialize(libraryPath string) (string, error) {
	fmt.Println("Initializing Quill library...\n")
	if libraryPath == "" {
		libraryPath = quillHelpers.GetDefaultLibraryPath()
	}
	libraryPath, err := createLibraryDir(libraryPath)
	if err != nil {
		return "", fmt.Errorf("error initializing directory: %w", err)
	}

	fmt.Printf("Initializing notebook...\n")
	notebookData, err := quillNotebooks.CreateNotebookDir(libraryPath)
	if err != nil {
		return "", fmt.Errorf("error creating notebook directory: %w", err)
	}
	_, err = quillNotebooks.CreateNotebook(notebookData)
	if err != nil {
		return "", fmt.Errorf("error creating notebook: %w", err)
	}

	fmt.Printf("Initializing page...\n")
	_, err = quillPages.CreatePageDir(notebookData.NotebookPath)
	if err != nil {
		return "", fmt.Errorf("error creating page directory: %w", err)
	}

	return libraryPath, nil
}
