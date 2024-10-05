package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", handler)

	// Listen on TCP
	addr := "127.0.0.1:3000"
	fmt.Println("Starting server at", addr)
	go func() {
		err := http.ListenAndServe(addr, nil)
		if err != nil {
			fmt.Println("Error starting server:", err)
		}
	}()
	fmt.Println("Server listening for TCP at", addr)

	select {}
}

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello, Go!")
}
