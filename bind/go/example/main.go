package main

import (
	"fmt"

	rust_binding "sample.com/rust_binding/go"
)

func main() {
	words := "how long is this?"
	cnt := rust_binding.Count(words)
	fmt.Printf("Count('%s'): %d\n", words, cnt)

	sum := rust_binding.Sum(7, 12)
	fmt.Printf("Sum(7, 12): %d\n", sum)

	concat := rust_binding.Concat("hello ", "world")
	fmt.Printf("Concat('hello ', 'world'): %s\n", concat)

	foo := rust_binding.Foo_new(17)
	fmt.Printf("multiplier: %d\n", foo.GetCount())
	fmt.Printf("foo_multiply(5): %d\n", rust_binding.Foo_multiply(foo, 5))

	foo.SetCount(12)
	fmt.Printf("multiplier: %d\n", foo.GetCount())
	fmt.Printf("foo_multiply(6): %d\n", rust_binding.Foo_multiply(foo, 6))
}
