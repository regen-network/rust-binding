const {sum, count, concat, foo_new, foo_multiply, foo_update} = require("./wasm/rust_wasm");

let total = sum(12, 14);
console.log(`Sum(12, 14): ${total}`);

let length = count("welcome to wasm");
console.log(`count("welcome to wasm"): ${length}`);

let long = concat("hello ", "world");
console.log(`concat("hello ", "world"): ${long}`);

let foo = foo_new(17)

console.log("multiplier: ", foo.count)
console.log("foo_multiply(5):", foo_multiply(foo, 5))

// update cannot be exported due to &mut issue
//foo_update(foo, 12)

// we can only modify the struct this way... or is there another way??
foo.count = 12

// let's just make a new one
console.log("multiplier: ", foo.count)
console.log("foo_multiply(6):", foo_multiply(foo, 6))

// try a work-around with mutability issue
let food = foo_update(foo, 5)
console.log("multiplier: ", food.count)
console.log("foo_multiply(4):", foo_multiply(food, 4))


// nice to cleanup
food.free()
