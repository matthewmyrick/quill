package quillHelpers

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"time"
)

func GenerateId(secretPhrase string, currentTime time.Time) (string, error) {
	h := hmac.New(sha256.New, []byte(secretPhrase))

	dataToHash := fmt.Sprintf("%s-%d", secretPhrase, currentTime.UnixNano())
	h.Write([]byte(dataToHash))

	return hex.EncodeToString(h.Sum(nil)), nil
}
