package quillPages

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	quillHelpers "quill-cli/pkg/quill/helpers"
	quillPagesTypes "quill-cli/pkg/quill/pages/types"
)

func CreatePage(data quillPagesTypes.PageData) (*quillPagesTypes.Page, error) {
	if data.Name == "" || data.PagePath == "" {
		return nil, fmt.Errorf("page name and library path cannot be empty")
	}
	secretKey := "klay-thompson"

	id, err := quillHelpers.GenerateId(secretKey, data.Name)
	if err != nil {
		return nil, fmt.Errorf("could not generate page ID: %w", err)
	}

	currentTime := time.Now()
	pg := &quillPagesTypes.Page{
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
		"type: Page\nid: %s\nname: %s\nrepo: %s\norg: %s\nbranch: %s\ntags: [%s]\ncreated: %s\nupdated: %s\ndescription: %s\n",
		pg.ID,
		pg.Name,
		pg.Repo,
		pg.Org,
		pg.Branch,
		strings.Join(pg.Tags, ", "),
		pg.Created.Format(time.RFC3339),
		pg.Updated.Format(time.RFC3339),
		pg.Description,
	)

	// Define the file path
	fileName := "page-" + pg.ID + ".quill"
	filePath := filepath.Join(data.PagePath, fileName)

	fmt.Printf(data.PagePath)
	fmt.Printf("Creating page file at: %s\n", filePath)

	// Check if file already exists
	if _, err := os.Stat(filePath); !os.IsNotExist(err) {
		return nil, fmt.Errorf("page file already exists at %s", filePath)
	}

	// Write the file
	err = os.WriteFile(filePath, []byte(content), 0644)
	if err != nil {
		return nil, fmt.Errorf("could not write page file: %w", err)
	}

	return pg, nil
}
