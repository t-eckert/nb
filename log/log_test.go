package log

import (
	"log"
	"testing"
	"time"
)

func TestFormatDate(t *testing.T) {
	given := time.Date(2018, time.July, 14, 0, 0, 0, 0, time.UTC)
	expected := "2018-07-14"

	actual := formatDate(given)

	if expected != actual {
		log.Fatalf("given: %s\nexpected: %s\nactual: %s\n", given, expected, actual)
	}
}
