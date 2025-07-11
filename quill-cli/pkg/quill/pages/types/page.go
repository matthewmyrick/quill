package quillPageTypes

import "time"

// Page represents the structure of the data in a .quill file.
type Page struct {
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

// PageData holds the arguments for creating a new page.
type PageData struct {
	Name        string
	Branch      string
	Repo        string
	Org         string
	Description string
	PagePath    string
	Tags        []string
}
