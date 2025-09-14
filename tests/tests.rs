use rustar::{oct2bin, extract_file};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oct2bin() {
        assert_eq!(oct2bin(b"10"), 8);
        assert_eq!(oct2bin(b"7"), 7);
    }
    #[test]
    fn test_extract_file() {
        let archive: &[u8] = include_bytes!("./sample.tar");

        // Note: tar headers use NUL-padded filenames
        let file = extract_file(archive, "hello.txt")
            .expect("file not found");


        assert_eq!(file, b"Hello World!\n");
    }
}