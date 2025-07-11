package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	quillInit "quill-cli/pkg/quill/init"
)

var libraryPath string

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initializes a notebook based on the current git repository",
	Long:  `Finds the .git repository, ensures the Quill library directory exists, reads the repo info, and creates a new notebook file.`,
	Run: func(cmd *cobra.Command, args []string) {
		_, err := quillInit.Initialize(libraryPath)
		if err != nil {
			fmt.Printf("Error initializing Quill library: %v\n", err)
			os.Exit(1)
		}
	},
}

func init() {
	initCmd.Flags().StringVarP(&libraryPath, "libraryPath", "p", "", "Specify a custom path for the Quill library")
}
