package quillHelpers

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
)

func GenerateId(secretPhrase string, name string) (string, error) {
	h := hmac.New(sha256.New, []byte(secretPhrase))

	dataToHash := fmt.Sprintf("%s-%s", secretPhrase, name)
	h.Write([]byte(dataToHash))

	return hex.EncodeToString(h.Sum(nil)), nil
}
