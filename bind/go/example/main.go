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

	// one way to construct it (auto-gen)
	foo := rust_binding.NewFoo()
	foo.SetCount(17)

	// more explicit (from foo_new rust call)
	foo = rust_binding.FooNew(17)

	fmt.Printf("multiplier: %d\n", foo.GetCount())
	fmt.Printf("foo_multiply(5): %d\n", rust_binding.FooMultiply(foo, 5))

	foo.SetCount(12)
	fmt.Printf("multiplier: %d\n", foo.GetCount())
	fmt.Printf("foo_multiply(6): %d\n", rust_binding.FooMultiply(foo, 6))
}
