package log

import (
	"os"
	"time"

	"github.com/t-eckert/nb/config"
)

func LogPath(dayOffset int) (string, error) {
	notesDir, err := config.GetRootDir()
	if err != nil {
		return "", err
	}

	today := formatDate(time.Now().Add(time.Duration(dayOffset) * 24 * time.Hour))

	return notesDir + "/Log/" + today + ".md", nil
}

func DoesLogExist(logPath string) (bool, error) {
	_, err := os.Stat(logPath)

	if !os.IsNotExist(err) {
		return false, err
	}

	if err != nil {
		return false, nil
	}

	return true, nil
}

func GenerateNew(logPath string, dayOffset int) error {
	f, err := os.Create(logPath)
	if err != nil {
		return err
	}

	defer f.Close()

	_, err = f.WriteString(
		"# " + formateDateTitle(time.Now().Add(time.Duration(dayOffset)*24*time.Hour)) + "\n\n" +
			"## Tasks\n\n" + "## Notes\n\n")

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
