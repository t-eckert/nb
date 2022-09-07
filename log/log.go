package log

import (
	"fmt"
	"os"
	"strconv"
	"time"

	"github.com/t-eckert/nb/config"
	"github.com/t-eckert/nb/editor"
)

const offset = 0

type Config struct {
	Offset int
}

func Log(open bool, args ...string) error {
	config, err := parseArgs(args...)
	if err != nil {
		return fmt.Errorf("could not parse arguments: %v", err)
	}

	path, err := configurePath(config.Offset)
	if err != nil {
		return fmt.Errorf("could not fetch today's log: %v", err)
	}

	exists, err := exists(path)
	if err != nil {
		return fmt.Errorf("could not check if log exists: %v", err)
	}

	if !exists {
		if err := create(path, config.Offset); err != nil {
			return fmt.Errorf("could not create new log: %v", err)
		}
	}

	if !open {
		return nil
	}

	if err = editor.Open(path); err != nil {
		return fmt.Errorf("could not open %s: %v", path, err)
	}
	return nil
}

func parseArgs(args ...string) (*Config, error) {
	if len(args) == 0 {
		return &Config{Offset: 0}, nil
	}

	offset, err := strconv.Atoi(args[0])
	if err != nil {
		return nil, err
	}

	return &Config{Offset: offset}, nil
}

func configurePath(offset int) (string, error) {
	root, err := config.GetRootDir()
	if err != nil {
		return "", err
	}

	return root + "/Log/" + date(offset).Format("2006-01-02") + ".md", nil
}

func exists(path string) (bool, error) {
	_, err := os.Stat(path)
	if err == nil {
		return true, nil
	}

	// Check if the stat err is something other than "does not exist".
	if !os.IsNotExist(err) {
		return false, err
	}

	return false, nil
}

func create(path string, offset int) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}

	defer f.Close()

	template := fmt.Sprintf(`# %s 

## Tasks


`, formateDateTitle(time.Now().Add(time.Duration(offset)*24*time.Hour)))

	_, err = f.WriteString(template)

	if err != nil {
		return err
	}

	return nil
}

func date(offset int) time.Time {
	return time.Now().Add(time.Duration(offset) * 24 * time.Hour)
}

func formatDate(t time.Time) string {
	return t.Format("2006-01-02")
}

func formateDateTitle(t time.Time) string {
	return t.Format("2 Jan 2006")
}
