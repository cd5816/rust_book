// Chapter 9.1 - Unrecoverable Errors with panic!
//
// Purpose:
// - Use panic behavior when continuing would be invalid or unsafe.
// - Panic stops this execution path immediately.
//
// Mental model:
// - panic! is a "hard stop" for this run of the program.
// - Default behavior unwinds the stack; release profile can be set to abort.
//
// Tool choice:
// - Use panic! for unrecoverable states (broken invariants, impossible states).
// - Prefer recoverable handling (Option/Result) when callers can decide what to do.
//
// Probe A (direct panic):
// fn main() {
//     panic!("crash and burn");
// }
// Expected: panic message + source location (file:line:column).
//
// Probe B (this program): panic from bounds checking in library code.
// - Indexing with [] expects a valid element.
// - Out-of-bounds indexing triggers panic to prevent undefined behavior.
//
// Backtrace rule:
// - Run with: RUST_BACKTRACE=1 cargo run --bin panic
// - Scan frames until the first line in your code; start debugging there.
//
// Optional release profile note (Cargo.toml):
// [profile.release]
// panic = 'abort'
// - abort does less cleanup work than unwind and terminates immediately.

fn main() {
    let v = vec![1, 2, 3];
    v[99]; // Intentional panic for learning.
}
