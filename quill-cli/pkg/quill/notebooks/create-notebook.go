package quillNotebooks

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	quillHelpers "quill-cli/pkg/quill/helpers"
	quillNotebooksTypes "quill-cli/pkg/quill/notebooks/types"
)

func CreateNotebook(data quillNotebooksTypes.NotebookData) (*quillNotebooksTypes.Notebook, error) {
	if data.Name == "" || data.NotebookPath == "" {
		return nil, fmt.Errorf("notebook name and library path cannot be empty")
	}
	secretKey := "stephen-curry"

	id, err := quillHelpers.GenerateId(secretKey, data.Name)
	if err != nil {
		return nil, fmt.Errorf("could not generate notebook ID: %w", err)
	}

	currentTime := time.Now()
	nb := &quillNotebooksTypes.Notebook{
		ID:          id,
		Name:        data.Name,
		Repo:        data.Repo,
		Org:         data.Org,
		Tags:        data.Tags,
		Created:     currentTime,
		Updated:     currentTime,
		Description: data.Description,
	}

	// Format the content to be written to the file
	content := fmt.Sprintf(
		"type: Notebook\nid: %s\nname: %s\nrepo: %s\norg: %s\ntags: [%s]\ncreated: %s\nupdated: %s\ndescription: %s\n",
		nb.ID,
		nb.Name,
		nb.Repo,
		nb.Org,
		strings.Join(nb.Tags, ", "),
		nb.Created.Format(time.RFC3339),
		nb.Updated.Format(time.RFC3339),
		nb.Description,
	)

	// Define the file path
	fileName := "notebook-" + nb.ID + ".quill"
	filePath := filepath.Join(data.NotebookPath, fileName)

	fmt.Printf(data.NotebookPath)
	fmt.Printf("Creating notebook file at: %s\n", filePath)

	// Check if file already exists
	if _, err := os.Stat(filePath); !os.IsNotExist(err) {
		fmt.Println("Notebook file already exists at:", filePath)
		return nb, nil
	}

	// Create the notebook file
	err = os.WriteFile(filePath, []byte(content), 0644)
	if err != nil {
		return nil, fmt.Errorf("could not write notebook file: %w", err)
	}

	return nb, nil
}
