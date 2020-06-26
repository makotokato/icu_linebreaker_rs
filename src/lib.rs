const UBRK_LINE: u32 = 2;
const U_ZERO_ERROR: u32 = 0;

#[link(name = "icuuc")]
extern "C" {
    #[link_name = "ubrk_open_67"]
    fn ubrk_open(
        iterator_type: u32,
        locale: *const u8,
        text: *const u16,
        textLength: i32,
        status: *mut u32,
    ) -> u64;
    #[link_name = "ubrk_next_67"]
    fn ubrk_next(bi: u64) -> i32;
    #[link_name = "ubrk_close_67"]
    fn ubrk_close(bi: u64);
}

pub struct LineBreakIterator {
    iterator_handle: u64,
}

impl Iterator for LineBreakIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator_handle == 0 {
            return None;
        }

        unsafe {
            let index = ubrk_next(self.iterator_handle);
            if index < 0 {
                ubrk_close(self.iterator_handle);
                self.iterator_handle = 0;
                return None;
            }
            Some(index as usize)
        }
    }
}

impl LineBreakIterator {
    pub fn new(input: &[u16]) -> LineBreakIterator {
        let mut status: u32 = U_ZERO_ERROR;
        let locale: [u8; 3] = [0x65, 0x6e, 0x00];
        unsafe {
            let handle = ubrk_open(
                UBRK_LINE,
                locale.as_ptr(),
                input.as_ptr(),
                input.len() as i32,
                &mut status,
            );
            LineBreakIterator {
                iterator_handle: handle,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LineBreakIterator;

    #[test]
    fn linebreak() {
        let s = "hello world";
        let s_utf16: Vec<u16> = s.encode_utf16().map(|x| x).collect();
        let mut iter = LineBreakIterator::new(&s_utf16);
        assert_eq!(Some(6), iter.next());
        assert_eq!(Some(11), iter.next());
        assert_eq!(None, iter.next());
    }
}
