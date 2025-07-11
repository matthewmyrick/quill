package quillNotebooksTypes

import "time"

// Notebook represents the structure of the data in a .quill file.
type Notebook struct {
	ID          string
	Name        string
	Branch      string
	Repo        string
	Org         string
	Tags        []string
	Created     time.Time
	Updated     time.Time
	Description string
}

// NotebookData holds the arguments for creating a new notebook.
type NotebookData struct {
	Name         string
	Branch       string
	Repo         string
	Org          string
	Description  string
	NotebookPath string
	Tags         []string
}
