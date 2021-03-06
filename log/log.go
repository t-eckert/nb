package log

import (
	"fmt"
	"os"
	"time"

	"github.com/t-eckert/nb/config"
)

func LogPath(dayOffset int) (string, error) {
	root, err := config.GetRootDir()
	if err != nil {
		return "", err
	}

	today := formatDate(time.Now().Add(time.Duration(dayOffset) * 24 * time.Hour))

	return root + "/Log/" + today + ".md", nil
}

func DoesLogExist(logPath string) (bool, error) {
	_, err := os.Stat(logPath)
	if err == nil {
		return true, nil
	}

	// Check if the stat err is something other than "does not exist".
	if !os.IsNotExist(err) {
		return false, err
	}

	return false, nil
}

func GenerateNew(logPath string, dayOffset int) error {
	f, err := os.Create(logPath)
	if err != nil {
		return err
	}

	defer f.Close()

	template := fmt.Sprintf(`# %s 

## Tasks


`, formateDateTitle(time.Now().Add(time.Duration(dayOffset)*24*time.Hour)))

	_, err = f.WriteString(template)

	if err != nil {
		return err
	}

	return nil
}

func formatDate(t time.Time) string {
	return t.Format("2006-01-02")
}

func formateDateTitle(t time.Time) string {
	return t.Format("2 Jan 2006")
}
