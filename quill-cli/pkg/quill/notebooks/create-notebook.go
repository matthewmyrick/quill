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

func createNotebook(data quillNotebooksTypes.NotebookData) (*quillNotebooksTypes.Notebook, error) {
	if data.Name == "" || data.NotebookPath == "" {
		return nil, fmt.Errorf("notebook name and library path cannot be empty")
	}
	secretKey := "stephen-curry"
	currentTime := time.Now()
	id, err := quillHelpers.GenerateId(secretKey, currentTime)
	if err != nil {
		return nil, fmt.Errorf("could not generate notebook ID: %w", err)
	}

	nb := &quillNotebooksTypes.Notebook{
		ID:          id,
		Name:        data.Name,
		Branch:      data.Branch,
		Repo:        data.Repo,
		Org:         data.Org,
		Tags:        data.Tags,
		Created:     currentTime,
		Updated:     currentTime,
		Description: data.Description,
	}

	// Format the content to be written to the file
	content := fmt.Sprintf(
		"type: Notebook\nid: %s\nname: %s\nbranch: %s\nrepo: %s\norg: %s\ntags: [%s]\ncreated: %s\nupdated: %s\ndescription: %s\n",
		nb.ID,
		nb.Name,
		nb.Branch,
		nb.Repo,
		nb.Org,
		strings.Join(nb.Tags, ", "),
		nb.Created.Format(time.RFC3339),
		nb.Updated.Format(time.RFC3339),
		nb.Description,
	)

	// Define the file path
	fileName := fmt.Sprintf("%s-%s.quill", data.Org, data.Repo)
	filePath := filepath.Join(data.NotebookPath, fileName)

	fmt.Printf(data.NotebookPath)
	fmt.Printf("Creating notebook file at: %s\n", filePath)

	// Check if file already exists
	if _, err := os.Stat(filePath); !os.IsNotExist(err) {
		return nil, fmt.Errorf("notebook file already exists at %s", filePath)
	}

	// Write the file
	err = os.WriteFile(filePath, []byte(content), 0644)
	if err != nil {
		return nil, fmt.Errorf("could not write notebook file: %w", err)
	}

	return nb, nil
}
