package quillHelpers

import (
	"os"
	"path/filepath"
)

func GetDefaultLibraryPath() string {
	// os.UserConfigDir() returns the default directory for user-specific
	// configuration files.
	// - Linux/macOS: ~/.config
	// - Windows: %APPDATA%
	configDir, _ := os.UserConfigDir()
	return filepath.Join(configDir, "quill", "library")
}
