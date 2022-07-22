package cmd

import (
	"log"

	"github.com/spf13/cobra"
	"github.com/t-eckert/nb/editor"
	noteLog "github.com/t-eckert/nb/log"
)

// logCmd represents the log command
var logCmd = &cobra.Command{
	Use:   "log",
	Short: "Create and edit daily logs.",
	Long:  ``,

	Run: func(cmd *cobra.Command, args []string) {
		fileName, err := noteLog.LogPath(0)
		if err != nil {
			log.Fatalf("could not fetch today's log: %v", err)
		}

		logExists, err := noteLog.DoesLogExist(fileName)
		if err != nil {
			log.Fatalf("could not check if log exists: %v", err)
		}

		if !logExists {
			noteLog.GenerateNew(fileName, 0)
		}

		if err = editor.Open(fileName); err != nil {
			log.Fatalf("could not open %s: %v", fileName, err)
		}
	},
}

func init() {
	rootCmd.AddCommand(logCmd)
	logCmd.PersistentFlags().Int("Offset", 0, "Offset")
}
