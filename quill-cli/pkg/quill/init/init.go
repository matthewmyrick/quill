package quillInit

import (
	"fmt"
	"os"
)

func Initialize() {
	_, err := createLibraryDirectory()
	if err != nil {
		fmt.Println("Error initializing directory:", err)
		os.Exit(1)
	}
	fmt.Println("Quill library directory initialized successfully.")
}
