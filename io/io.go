package io

import "fmt"

func CreateFile(filePath string) error {
	return nil
}

func AppendToFile(filePath string, content string) error {
	if !DoesFileExist(filePath) {
		return fmt.Errorf("file not found %s", filePath)
	}

	return nil
}

func DoesFileExist(filePath string) bool {
	return false
}
