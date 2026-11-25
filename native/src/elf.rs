// spec: gate-sdk/SPEC.md §check-binary-strings — the ELF64 little-endian reader: section offsets,
// sizes, types and flags only, enough to name the byte ranges a scan reads
use std::ops::Range;

const HEADER: usize = 64;
const SECTION: usize = 64;
const SHT_NULL: u32 = 0;
const SHT_NOBITS: u32 = 8;
const SHF_EXECINSTR: u64 = 0x4;

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn u64_at(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
}

// spec: gate-sdk/SPEC.md §check-binary-strings — the magic, class 2 and data 1; any other file,
// a 32-bit or big-endian ELF among them, is read whole
pub fn is_elf64le(bytes: &[u8]) -> bool {
    bytes.len() >= 6 && bytes[..4] == *b"\x7fELF" && bytes[4] == 2 && bytes[5] == 1
}

fn span(offset: u64, size: u64, len: usize) -> Option<Range<usize>> {
    let start = usize::try_from(offset).ok()?;
    let end = start.checked_add(usize::try_from(size).ok()?)?;
    (end <= len).then_some(start..end)
}

// spec: gate-sdk/SPEC.md §check-binary-strings — the byte ranges a scan reads: each section that
// occupies the file and does not execute, in section-table order, else the whole file; `Err`
// names what in an ELF64 little-endian file does not parse or points outside it
pub fn scan_ranges(bytes: &[u8]) -> Result<Vec<Range<usize>>, String> {
    if !is_elf64le(bytes) {
        let whole = 0..bytes.len();
        return Ok(vec![whole]);
    }
    if bytes.len() < HEADER {
        return Err(format!("ELF header truncated at {} of {} bytes", bytes.len(), HEADER));
    }
    let bad = |what: &str| format!("ELF {} outside the file", what);
    let shoff = u64_at(bytes, 0x28).ok_or_else(|| bad("header"))?;
    let shentsize = usize::from(u16_at(bytes, 0x3A).ok_or_else(|| bad("header"))?);
    let mut shnum = u64::from(u16_at(bytes, 0x3C).ok_or_else(|| bad("header"))?);
    if shoff == 0 {
        return Ok(Vec::new());
    }
    if shentsize < SECTION {
        return Err(format!("ELF section header size {} is below {}", shentsize, SECTION));
    }
    let base = usize::try_from(shoff).map_err(|_| bad("section table"))?;
    // spec: gate-sdk/SPEC.md §check-binary-strings — extended numbering: a zero count with a table
    // present carries the real count in section 0's size
    if shnum == 0 {
        shnum = u64_at(bytes, base.checked_add(0x20).ok_or_else(|| bad("section table"))?)
            .ok_or_else(|| bad("section table"))?;
    }
    let table = usize::try_from(shnum)
        .ok()
        .and_then(|n| n.checked_mul(shentsize))
        .and_then(|t| t.checked_add(base))
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| bad("section table"))?;
    let mut out = Vec::new();
    for at in (base..table).step_by(shentsize) {
        let kind = u32_at(bytes, at + 4).ok_or_else(|| bad("section header"))?;
        let flags = u64_at(bytes, at + 8).ok_or_else(|| bad("section header"))?;
        let offset = u64_at(bytes, at + 0x18).ok_or_else(|| bad("section header"))?;
        let size = u64_at(bytes, at + 0x20).ok_or_else(|| bad("section header"))?;
        if kind == SHT_NULL || kind == SHT_NOBITS || flags & SHF_EXECINSTR != 0 {
            continue;
        }
        out.push(span(offset, size, bytes.len()).ok_or_else(|| bad("section"))?);
    }
    Ok(out)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-binary-strings — a constructed image: the header, then each
    // section's bytes, then a section table whose first entry is the null section
    pub fn image(sections: &[(u32, u64, &[u8])]) -> Vec<u8> {
        let mut b = vec![0u8; HEADER];
        b[..6].copy_from_slice(b"\x7fELF\x02\x01");
        let mut placed = Vec::new();
        for (kind, flags, data) in sections {
            placed.push((*kind, *flags, b.len() as u64, data.len() as u64));
            b.extend_from_slice(data);
        }
        let shoff = b.len() as u64;
        b[0x28..0x30].copy_from_slice(&shoff.to_le_bytes());
        b[0x3A..0x3C].copy_from_slice(&(SECTION as u16).to_le_bytes());
        b[0x3C..0x3E].copy_from_slice(&((placed.len() + 1) as u16).to_le_bytes());
        b.extend_from_slice(&[0u8; SECTION]);
        for (kind, flags, offset, size) in placed {
            let mut h = [0u8; SECTION];
            h[4..8].copy_from_slice(&kind.to_le_bytes());
            h[8..16].copy_from_slice(&flags.to_le_bytes());
            h[0x18..0x20].copy_from_slice(&offset.to_le_bytes());
            h[0x20..0x28].copy_from_slice(&size.to_le_bytes());
            b.extend_from_slice(&h);
        }
        b
    }

    #[test]
    fn an_image_yields_its_data_sections_and_never_code_or_nobits() {
        let img = image(&[(1, 0x6, b"code"), (1, 0x2, b"rodata"), (SHT_NOBITS, 0x3, b""), (1, 0x3, b"data")]);
        let got: Vec<&[u8]> = scan_ranges(&img).unwrap().into_iter().map(|r| &img[r]).collect();
        assert_eq!(got, vec![&b"rodata"[..], &b"data"[..]]);
    }

    #[test]
    fn a_file_that_is_no_elf64_le_is_read_whole() {
        for whole in [&b"plain text"[..], &b"\x7fELF\x01\x01 a 32-bit one"[..]] {
            let all = 0..whole.len();
            assert_eq!(scan_ranges(whole).unwrap(), vec![all]);
        }
    }

    #[test]
    fn a_truncated_header_and_a_section_past_the_end_are_refused() {
        assert!(scan_ranges(b"\x7fELF\x02\x01short").is_err());
        let mut img = image(&[(1, 0x2, b"rodata")]);
        let at = img.len() - SECTION + 0x20;
        img[at..at + 8].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(scan_ranges(&img).is_err());
        let cut = image(&[(1, 0x2, b"rodata")]);
        assert!(scan_ranges(&cut[..cut.len() - 1]).is_err());
    }
}
