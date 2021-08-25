package config

const editor = "nvim"
const rootDir = "~/notebook"

func GetEditor() (string, error) {
	return editor, nil
}

func GetRootDir() (string, error) {
	return rootDir, nil
}
