# u32err
This crate implements the [`core::ops::Try`](https://doc.rust-lang.org/nightly/core/ops/trait.Try.html) trait with a thin wrapper over [`u32`](https://doc.rust-lang.org/nightly/core/primitive.u32.html).

You may use it to implement ergonomic error handling for FFI functions that return non-zero values on failure,
or as a lightweight [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html).

## Example

```rust
use u32err::ErrCode;
extern "C" {
    /// This is a function that does something (via FFI).
    ///
    /// It returns either a 0 on success, or a non-zero number on failure.
    /// The real FFI signature of this function returns [`u32`], but the types are compatible.
    fn returns_zero_on_success() -> ErrCode;
}

fn foo() -> ErrCode {
    unsafe {
        returns_zero_on_success()?;
    }
    ErrCode(0)
}
```