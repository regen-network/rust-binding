package main

import (
	"fmt"

	rust_binding "sample.com/rust_binding/go"
)

func main() {
	words := "how long is this?"
	cnt := rust_binding.Count(words)
	fmt.Printf("Count('%s'): %d\n", words, cnt)
}