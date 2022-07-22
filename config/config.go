package config

import (
	"fmt"
	"os"
)

const (
	Editor = "nvim"
	Port   = 3000
)

func GetRootDir() (string, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return "", nil
	}

	return fmt.Sprintf("%s/Notebook", home), nil
}
