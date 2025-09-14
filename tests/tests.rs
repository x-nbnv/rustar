use rustar::{oct2bin, extract_file, TarIter};

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    // Helper: generate a tar archive with one file
    fn make_tar(filename: &str, contents: &[u8]) -> Vec<u8> {
        let mut header = [0u8; 512];

        // Filename (NUL padded)
        let name_bytes = filename.as_bytes();
        header[..name_bytes.len()].copy_from_slice(name_bytes);

        // File size (octal, NUL padded)
        let size_str = format!("{:o}", contents.len());
        let size_bytes = size_str.as_bytes();
        header[0x7c..0x7c + size_bytes.len()].copy_from_slice(size_bytes);

        // USTAR magic
        header[257..257 + 5].copy_from_slice(b"ustar");

        // Checksum: fill with spaces first
        for i in 148..156 {
            header[i] = b' ';
        }
        let chksum: u32 = header.iter().map(|&b| b as u32).sum();
        let chksum_str = format!("{:06o}\0 ", chksum);
        header[148..148 + chksum_str.len()].copy_from_slice(chksum_str.as_bytes());

        let mut archive = Vec::new();
        archive.extend_from_slice(&header);

        // File contents (pad to 512)
        archive.extend_from_slice(contents);
        let padding = (512 - (contents.len() % 512)) % 512;
        archive.extend(core::iter::repeat(0).take(padding));

        // End of archive (two zero blocks)
        archive.extend([0u8; 1024]);

        archive
    }

    #[test]
    fn test_oct2bin() {
        assert_eq!(oct2bin(b"10"), 8);
        assert_eq!(oct2bin(b"7"), 7);
        assert_eq!(oct2bin(b"00010"), 8); // leading zeros
        assert_eq!(oct2bin(b""), 0);      // empty string
    }

    #[test]
    fn test_extract_file_from_sample() {
        let archive: &[u8] = include_bytes!("./sample.tar");

        let file = extract_file(archive, "hello.txt")
            .expect("file not found");

        assert_eq!(file, b"Hello World!\n");
    }

    #[test]
    fn test_extract_file_from_generated() {
        let tar = make_tar("liz.txt", b"hello owo\n");

        let file = extract_file(&tar, "liz.txt")
            .expect("file not found in generated tar");

        assert_eq!(file, b"hello owo\n");
    }

    #[test]
    fn test_extract_missing_file() {
        let tar = make_tar("only.txt", b"data");
        assert!(extract_file(&tar, "missing.txt\0").is_none());
    }

    #[test]
    fn test_extract_empty_file() {
        let tar = make_tar("in the grim darkness of the future.txt", b"");
        let file = extract_file(&tar, "in the grim darkness of the future.txt")
            .expect("empty file not found");
        assert!(file.is_empty());
    }

    #[test]
    fn test_iter_multiple_files() {
        // Concatenate two tars manually to simulate multi-file archive
        let mut tar1 = make_tar("puppyboy.txt", b"OwO\n");
        let mut tar2 = make_tar("puppygirl.txt", b"^W^\n");
        // remove the ending zero blocks from tar1
        tar1.truncate(tar1.len() - 1024);
        tar1.append(&mut tar2);

        let files: Vec<_> = TarIter::new(&tar1)
            .map(|hdr| {
                let raw: &[u8] = hdr.name.as_bytes(); // ensure it's &[u8] via as_bytes
                let trimmed = raw.split(|&b| b == 0) // split on NUL
                    .next()
                    .unwrap();
                String::from_utf8_lossy(trimmed).to_string()
            })
            .collect();



        assert!(files.contains(&"puppyboy.txt".to_string()));
        assert!(files.contains(&"puppygirl.txt".to_string()));

    }
}
