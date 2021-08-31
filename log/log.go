package log

import (
"time"

"github.com/t-eckert/nb/config"
)

func FetchTodaysLog() (string, error) {
	notesDir, err := config.GetRootDir()
	if err != nil {
		return "", err
	}

	today := formatDate(time.Now())

	return notesDir + "/Log/"+ today + ".md", nil
}

func formatDate(t time.Time) string {
	return t.Format("2006-01-02")	
}
