from rust_ffi import sum, count, concat, foo_new, foo_multiply, foo_update

print("Sum(4, 5):", sum(4, 5))

print("Count('two words'):", count("two words"))

print("Concat('hello ', 'world')", concat('hello ', 'world'))

foo = foo_new(17)

print("multiplier: ", foo.count)
print("foo_multiply(5):", foo_multiply(foo, 5))

foo_update(foo, 12)

print("multiplier: ", foo.count)
print("foo_multiply(6):", foo_multiply(foo, 6))

foo.count = 5
print("multiplier: ", foo.count)
print("foo_multiply(4):", foo_multiply(foo, 4))
