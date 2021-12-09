# autovec

Some simple auto-vectorization cases. Created to assist in solving the [Rust #85265](https://github.com/rust-lang/rust/issues/85265) basic vectorization regression issue.

There's no magic. The idea is to eyeball the output assembly to see if it's efficient.


## Usage

We can simply copy paste [lib.rs](https://github.com/shampoofactory/autovec/blob/main/src/lib.rs) into Matt Godbolt's wonderful [Compiler Explorer](https://rust.godbolt.org/) and set the appropriate compiler flags. Here are links for [x64](https://rust.godbolt.org/z/KcErr3s6G) and [aarch64](https://rust.godbolt.org/z/seMhr3sh9) that use Rust 1.47.

To compile with a local rust dev build, assuming the appropriate `path/to/my/toolchain/sysroot` substitution, we can use something like:

```Bash
$ rustup toolchain link my-toolchain path/to/my/toolchain/sysroot
$ rustup default my-toolchain
```
```Bash
$ cargo rustc --release -- -C opt-level=3 -C codegen-units=1  -C target-cpu=skylake --emit=asm
```
```Bash
$ cargo rustc --release -- -C opt-level=3 -C codegen-units=1 --target=aarch64-unknown-linux-gnu --emit=asm
```

The output should appear in `/target/release/deps/autovec-HASH.s`. For better or worse, we get AT&T assembly syntax.


## Output 1.47 analysis: inefficient cases

Although I'm referencing 1.47 as a non-regressed state, there are still inefficient cases.

Compound instructions:
```
case_4_i32_add_sub_com
case_vec4_add_sub_to_cancel_c
case_vec4_add_to_mul_15
case_vec4_add_to_mul_16
```

Non xmm/ ymm register sized types:
```
Vec3 manipulations *
```

* see [issue 91447](https://github.com/rust-lang/rust/issues/91447), Clang coalesces the first two `vmovss` instructions to `vmovsd`.


## Output analysis

The output is intended to be eyeballed by those familiar with x64/ aarch64.

In more practical terms, give or take the 1.47 inefficient cases mentioned above, we are looking for output that returns us to Rust 1.47.0.

## Output case analysis: 'case_4_f32_add'

Source for reference:
```Rust
pub fn case_4_f32_add(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}
```

x64: Rust 1.47: efficient:
```
example::case_4_i32_add:
        mov     rax, rdi
        vmovdqu xmm0, xmmword ptr [rdx]
        vpaddd  xmm0, xmm0, xmmword ptr [rsi]
        vmovdqu xmmword ptr [rdi], xmm0
        ret
```
x64: Rust 1.57: inefficient:
```
example::case_4_i32_add:
        mov     r8, rcx
        add     ecx, esi
        shr     rsi, 32
        lea     eax, [rdx + rdi]
        shr     rdi, 32
        shr     r8, 32
        shr     rdx, 32
        add     edx, edi
        add     esi, r8d
        shl     rsi, 32
        or      rcx, rsi
        shl     rdx, 32
        or      rax, rdx
        mov     rdx, rcx
        ret
```

aarch64: Rust 1.47: efficient:
```
example::case_4_i32_add:
        ldr     q0, [x0]
        ldr     q1, [x1]
        add     v0.4s, v1.4s, v0.4s
        str     q0, [x8]
        ret
```

aarch64: Rust 1.57: inefficient:
```
example::case_4_i32_add:
        extr    x8, x1, x0, #32
        lsr     x9, x1, #32
        extr    x10, x3, x2, #32
        lsr     x11, x3, #32
        fmov    d0, x8
        fmov    d1, x9
        zip1    v0.4s, v1.4s, v0.4s
        fmov    d1, x10
        fmov    d2, x11
        zip1    v1.4s, v2.4s, v1.4s
        add     v0.2s, v1.2s, v0.2s
        ushll   v0.2d, v0.2s, #0
        add     w0, w2, w0
        add     w1, w3, w1
        mov     x8, v0.d[1]
        fmov    x9, d0
        bfi     x1, x9, #32, #32
        bfi     x0, x8, #32, #32
        ret
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
