package config

const editor = "nvim"
const rootDir = "/Users/thomaseckert/Notebook"

func GetEditor() (string, error) {
	return editor, nil
}

func GetRootDir() (string, error) {
	return rootDir, nil
}
