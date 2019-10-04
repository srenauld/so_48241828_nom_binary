extern crate nom;

const MAGIC:[u8;5] = [0x65, 0x51, 0x48, 0x54, 0x52];

pub fn extract_from_data(data: &mut [u8]) -> Option<Vec<u8>> {
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn successfully_parses_first_block() {
        let mut data:[u8;10] = [0x65, 0x51, 0x48, 0x54, 0x52, 0x01, 0x02, 0x03, 0xff, 0x00];
        let expected:Vec<u8> = vec![0x01, 0x02, 0x03];
        let output:Option<Vec<u8>> = extract_from_data(&mut data);
        assert_eq!(output, Some(expected));
    }
}