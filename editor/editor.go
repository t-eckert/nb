package editor

import (
	"os"
	"os/exec"

	"github.com/t-eckert/nb/config"
)

func Open(filePath string) error {
	cmd := exec.Command(config.Editor, filePath)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout

	return cmd.Run()
}
