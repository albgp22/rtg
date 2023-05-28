package main

import (
	"net/http"
)

func users(w http.ResponseWriter, req *http.Request) {
	// Responds a 200 status code with a JSON body containing {"result": "ok"}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"result":"ok"}`))
}

func main() {

	http.HandleFunc("/api/v1/users", users)

	http.ListenAndServe(":8080", nil)
}
