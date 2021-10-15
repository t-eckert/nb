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
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,

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

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// logCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// logCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
