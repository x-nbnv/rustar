
# rustar (really unproductive system tape archive)

## What is rustar?  
**RUSTAR** (Rust + USTAR) is a minimal, safe (hopefully), and `#![no_std]`-friendly library for parsing [USTAR](https://en.wikipedia.org/wiki/Tar_(computing)#UStar_format) archives.  
It is designed primarily for Rust-Based **OS** development, in the case one might bundle their kernel modules, init programs, or resources in a `.tar` and extract them at runtime.


---

## Why use rustar for an OS?
Using RUSTAR as your filesystem format handler has some big advantages:
- **Zero external deps**: Works in `#![no_std]`, requires only `alloc` if you need owned `Vec<u8>`.  
- **Cross-tested**: Same code works on bare metal (e.g your OS kernel) and on Linux/Win with `cargo test`.

---

## Features
- purely rust based
- `#![no_std]` compatible  
- iterator function (`TarIter`) to walk over headers  
- zero-mem-copy file lookup (`tar_lookup`)  
- (optional) owned extraction (`extract_file`)  
- Tested with standard `tar --format=ustar` archives  

---

## Example in  OS kernel
This is an example implementation for it in an OS kernel, so same rules apply to a basic ```no_std``` environment:
```rust
for header in rustar::TarIter::new(archive) {
    println!("file: {} ({} bytes)", header.name, header.size);
}

if let Some(init_bin) = rustar::extract_file(archive, "init.bin") {
    // load and run init program…
}
```

---

## Example (testing on Linux)
You can include a tar file directly in your unit tests (this is the included unit test):

```rust
#[test]
fn test_extract_file() {
    let archive: &[u8] = include_bytes!("../tests/sample.tar");
    let file = rustar::extract_file(archive, "hello.txt")
        .expect("file not found");

    assert_eq!(file, b"Hello World!\n");
}
```

Create an (already provided) test tar with:

```bash
echo "Hello World!" > hello.txt
tar --format=ustar -cf tests/sample.tar hello.txt
```

Run tests:

```bash
cargo test
```

---

## Roadmap
- Add write support (create tar archives in `no_std`) , although i may already switch over my OS to fat12 or something by then
- Expose a safe `TarHeader::contents(&self, archive: &[u8])` helper  
- Support reading from block devices instead of memory slices (this would make life much easier but im lazy)

---

## License

this project uses the MIT License, in full effect. give me credit via name/username :3 (if you want)

---

<blockquote>
<table align="right">
<tr>
<td align="right">

Brought to you by your favourite puppygirl **Liz** 🐾  

</td>
<td>

<a href="https://github.com/x-nbnv">
  <img src="https://avatars.githubusercontent.com/u/65957437?v=4"
       width="140"
       height="140"
       style="border-radius:50%;"/>
</a>  
<p align="center"><sub><a href="https://github.com/x-nbnv">x-nbnv</a></sub></p>

</td>
</tr>
</table>
</blockquote>
