package cmd

import (
	"log"

	"github.com/spf13/cobra"
	noteLog "github.com/t-eckert/nb/log"
)

var logCmd = &cobra.Command{
	Use:   "log",
	Short: "Create and edit daily logs.",
	Long: `The log command will create and open a daily log file based on a template.

The offset flag may be passed to edit a daily log offset by the given number of days.
If a log already exists for a given day, it will be opened.`,

	Run: func(cmd *cobra.Command, args []string) {
		open, err := cmd.Flags().GetBool("open")
		if err != nil {
			log.Fatalf(err.Error())
		}
		if err := noteLog.Log(open, args...); err != nil {
			log.Fatal(err.Error())
		}
	},
}

func init() {
	rootCmd.AddCommand(logCmd)

	logCmd.PersistentFlags().BoolP("open", "o", true, "Whether or not to open the editor to the log.")
}
