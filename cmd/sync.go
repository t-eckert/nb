package cmd

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"time"

	"github.com/spf13/cobra"
	"github.com/t-eckert/nb/config"
)

var syncCmd = &cobra.Command{
	Use:   "sync",
	Short: "Sync the notebook to remote.",
	Long: `Sync will add all files to the commit, commit with the message
"Sync YYYY-MM-DD", and push the commit to the remote.`,

	Run: func(cmd *cobra.Command, args []string) {
		root, err := config.GetRootDir()
		if err != nil {
			log.Fatal(err)
		}

		err = os.Chdir(root)
		if err != nil {
			log.Fatal(err)
		}

		err = exec.Command("git", "add", ".").Run()
		if err != nil {
			log.Fatal(err)
		}

		err = exec.Command("git", "commit", "-m", fmt.Sprintf("Sync %s", time.Now().Format("2006-01-02"))).Run()
		if err != nil {
			log.Fatal(err)
		}

		err = exec.Command("git", "push").Run()
		if err != nil {
			log.Fatal(err)
		}
	},
}

func init() {
	rootCmd.AddCommand(syncCmd)
}
