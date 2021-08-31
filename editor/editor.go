package editor

import (
	"os"
	"os/exec"

	"github.com/t-eckert/nb/config"
)

func Open(filePath string) error {
	editorCommand, err := config.GetEditor()
	if err != nil {
		return err
	}

	cmd := exec.Command(editorCommand, filePath)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout

	if err = cmd.Run(); err != nil {
		return err
	}

	return nil
}
