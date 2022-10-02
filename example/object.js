let o = {
  message: "hello object",
};
console.log("o.message:", o.message);

o.message = "hello object again";
console.log("o.message (assigned):", o.message);

let o_cp = o;
o_cp.message = "hello object again by copy";
console.log("o.message (assigned by copy):", o.message);