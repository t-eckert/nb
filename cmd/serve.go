package cmd

import (
	"github.com/spf13/cobra"
	"github.com/t-eckert/nb/serve"
)

// serveCmd represents the serve command
var serveCmd = &cobra.Command{
	Use:   "serve",
	Short: "",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		serve.Serve()
	},
}

func init() {
	rootCmd.AddCommand(serveCmd)
}
