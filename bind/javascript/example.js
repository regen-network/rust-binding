const {sum, count, concat} = require("./wasm/rust_wasm");

let total = sum(12, 14);
console.log(`Sum(12, 14): ${total}`);

let length = count("welcome to wasm");
console.log(`count("welcome to wasm"): ${length}`);

let long = concat("hello ", "world");
console.log(`concat("hello ", "world"): ${long}`);
