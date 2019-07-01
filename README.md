# The Stack and Heap
- Both are parts of memory that are available to your code at runtime. 
- Different structure
- Stack: LIFO, adding data is called: `push`, removing data is call `pop`
- All data stored on the stack must have a know, fixed size.
- Data with an unknow size at compile time or a size might change must be store on Heap insead.
- Heap: when you put data on the heap, you request a certain amount of space, OS finds a empty spot in the heap that is big enough, marks it as being used, and return a pointer, which is address of that location -> allocating on the heap (allocating)
- Pushing values onto the stack is not considered allocating, because pointer is a known, fixed size.
- You can store the pointer on the stack, but when you want to actual data, you must follow the pointer.
- Accessing data in the heap is slower that accessing data on the stack because you have to follow a pointer to get there.
- jump around less in memory -> faster
- Allocating a large amount of space on the heap can alse take time.
- When your code calls a function, the value passed into the function (including pointer to data on the heap) and the function's local variables get push onto the stack.
- When the function is over, those values get popped off the stack.

# Ownership rules
- Each value in Rust has a value that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

# Variable scope
```rust
	let s = "hello";
```
variable `s` refers to a string literal.
```rust
	// is is not valid here, it's not yet declared.
	{
		let s = "hello";
		// s is valid from this point forward.
	}

	// this scope is over, s is no longer valid.
```

# String type
- String literal is immutable
- Not every string value can be know when we write our code.
- Rust has a second string type: `String`
- This type is allocated on the heap.
- Create a string
```rust
	let s = String::from("hello");
```
- In the case of literal string, we know the contents at compile time, so the text is hardcoded directly into the final executable. -> fast and efficient but immutable.
- With the `String` type, need to allocate an amount of memory on the heap, unknown at compile time.
- The memory must be requested from the OS at runtime.
- We need a way of returning this memory to the OS when we're done with our `String`
- In Rust, the memory is returned once the  variable that owns it get out of scope.
```rust
	let s1 = String::from("hello");
	let s2 = s1;
```
		s1
	-------------------------	-----------------
	|name 	| value		|	| index	| value	|
	|-------|---------------|	|---------------|	
	|ptr	| ----------------------> 0	| h	|
	|-------|---------------|	|---------------|
	|len	| 5		|	| 1	| e	|
	|-------|---------------|	|---------------|
	|capacit|y 5		|	| 2	| l	|
	--------|---------------	|---------------|
					| 3	| l	|
					|---------------|
					| 4    	| o	|
					-----------------
- The length is how much memory, in bytes
- When assign s1 to s2, the `String` data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that pointer refers to.
- When variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory for that variable.
- Rust considers `s1` to no longer valid after assign to s2.

```rust
	let s1 = String::from("hello");
	let s2 = s1;
	println!("{}", s1); // Error here!
```
- Shallow copy: just copy pointer, length, capacity, without copying the data.
- But Rust also invalidate the first variable -> Move
- `s1` has moved to `s2`
- Rust will never automatically create "deep" copies of your data.

# Clone
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("{}, {}", s1, s2);
```

- expensive

# Stack-Only Data: Copy
```rust
let x = 5;
let y = x;
println!("{}, {}", x, y);
```
- Types such as integers that have a known size at compile time are stored entirely on the stack, copies of the actual values are quickly to make.
- There is no different betwwen shallow and deep copy.
- x is still valid after copy to s2
- `Copy` trait
- if the type as the `Copy` trait, an older variable is still usable after assignment.
- Here are some of the types that are `Copy`:
	+ all the integer, u32
	+ Boolean type
	+ Floating point types, f64
	+ Character type, char.
	+ Tuples, if they only contain types that are also `Copy`.

# Ownership and Functions 
- Passing a varible to a function will move or copy, just as assignments does.

```rust
fn main() {
	let s = String::from("hello");
	takes_ownership(s);
	// s is no longer valid here
	let x = 5;
	makes_copy(x);
	// x is available here.
}

fn takes_ownership(s: String) {
	println!("{}", s);
}

fn makes_copy(x: i32) {
	println!("{}", x);
}
```

- The ownership of a variable follows the same pattern every time:
	+ assigning a value to another varible moves it.
	+ when the varible that includes data on the heap goes out the heap, the values will be dropped unless the data has been moved to be owned by another variable.

# References and Borrowing
```rust
fn main() {
	let s1 = String::from("hello");
	let len = calculate_length(&s1);

	println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
	s.len()
}
```

- The `&s1` syntax lets us create a reference that refers to the value of `s1` but not ownes it.
- Because it doesn't own it, the value it points to will no be dropped when the reference goes out of scope.
- `&` indicate that the type of the parameter `s` is a reference.

# Mutable References
```
fn main() {
	let mut s = String::from("hello");
	change(&mut s);
}

fn change(s: &mut String) {
	s.push_str(", world!")
}
```
- Restriction: you can have only one mutable reference to particular piece of data in a particular scope.
- As always, you can use curly backets to create a new scope, allowing for multiple mutable references

```rust 
	let mut s = String:: from("hello");
	{
		let r1 = &mut s;
	}
	let r2 = &mut s;
```
- You also cannot have a mutable reference while we have an immutable one.
- However, multiple immutable references a okay because no one who is just reading the data has the ability to affect anyone else's reading of the data.

# Dangling References
- In Rust, the compiler guarantees that references will never be dangling references.
- if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

```rust
	fn dangle() -> &String {
		let s = String::from("hello");
		&s // return a reference to the String, s
	}
	// s goes out of scope, and is dropped. Its memory goes away. 
	// Danger!
```

- Solution is return String directly
```rust
	fn no_dangle() -> String {
		let s = String::from("hello");
		s
	}
```
- Ownership is moved out, and nothing is deallocated.

# Slice Type
- type that doesn't have ownership
