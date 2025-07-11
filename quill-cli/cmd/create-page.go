package cmd

import (
	"github.com/spf13/cobra"
)

var createPageCmd = &cobra.Command{
	Use:   "create-page",
	Short: "Creates a new page in the notebook",
	Long:  `Creates a new page in the specified notebook, initializing it if necessary.`,
	Run: func(cmd *cobra.Command, args []string) {
	},
}

func init() {
	createPageCmd.Flags().StringVarP(&libraryPath, "title", "t", "", "Title of the page to create")
}
