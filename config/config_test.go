package config

import (
	"fmt"
	"testing"
)

func TestGetEditor(t *testing.T) {
	expected := "nvim"

	actual, _ := GetEditor()

	if expected != actual {
		fmt.Printf("Expected: %s\nReceived: %s\n", expected, actual)
	}
}

func TestGetRootDir(t *testing.T) {
	expected := "~/notebook"

	actual, _ := GetRootDir()

	if expected != actual {
		fmt.Printf("Expected: %s\nReceived: %s\n", expected, actual)
	}
}
