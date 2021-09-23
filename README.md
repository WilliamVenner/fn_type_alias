# ✨ `fn_type_alias`

A proc attribute macro that generates a type alias with the given identifier for the attributed function.

## Example

```rust
#[macro_use]
extern crate fn_type_alias;

#[type_alias(HelloWorldFn)] // The type alias will inherit its visibility from the function
pub(super) fn hello_world() {
	println!("hello world!");
}

#[type_alias(pub(crate), HelloWorldFn)] // The type alias will be pub(crate), but the function will be pub
pub fn hello_world() {
	println!("hello world!");
}
```

## Use Case

This macro is well suited for conditional compilation. For example, using the [`fn_abi`](https://crates.io/crates/fn_abi) macro:

```rust
#[macro_use]
extern crate fn_type_alias;

#[macro_use]
extern crate fn_abi;

#[abi(
	linux32 = "C",
	linux64 = "C",
	win32 = "thiscall",
	win64 = "stdcall"
)]
#[type_alias(HelloWorldFn)]
pub extern fn hello_world() {
	println!("hello world!");
}

// Expands to when building for Windows 64-bit:
pub type HelloWorldFn = extern "stdcall" fn();
pub extern "stdcall" fn hello_world() {
	println!("hello world!");
}
```