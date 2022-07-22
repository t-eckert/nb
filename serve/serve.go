package serve

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
	"path/filepath"

	"github.com/t-eckert/nb/config"
)

func Serve() error {
	root, err := config.GetRootDir()
	if err != nil {
		return err
	}

	static := http.FileServer(http.Dir("./static"))
	http.Handle("/static/", http.StripPrefix("/static/", static))

	fs := http.FileServer(http.Dir(root))
	http.Handle("/files/", http.StripPrefix("/files/", fs))

	http.HandleFunc("/", serveUI)

	port := fmt.Sprintf(":%d", config.Port)
	fmt.Printf("Serving on http://localhost%s", port)
	if err := http.ListenAndServe(port, nil); err != nil {
		log.Fatal(err)
	}

	return nil
}

type tpl struct {
	Files string
}

func serveUI(w http.ResponseWriter, r *http.Request) {

	files := `files!`

	t := tpl{files}

	index := filepath.Join("templates", "index.html")

	tmpl, _ := template.ParseFiles(index)
	tmpl.ExecuteTemplate(w, "index", t)
}
