#![no_std]

use core::result::Result;

use rand::RngCore;

#[allow(dead_code)]
struct CustomRng;

impl RngCore for CustomRng {
    fn next_u32(&mut self) -> u32 {
        // Triển khai phương thức tạo số ngẫu nhiên của riêng bạn
        0 // Đây chỉ là ví dụ
    }
    
    fn next_u64(&mut self) -> u64 {
        // Triển khai phương thức tạo số ngẫu nhiên của riêng bạn
        0 // Đây chỉ là ví dụ
    }

    fn fill_bytes(&mut self, _dest: &mut [u8]) {
        // Triển khai phương thức điền bytes ngẫu nhiên
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}

#[test]
fn test_custom_rng() {
    let mut rng = CustomRng;
    let mut buffer = [0u8; 8];
    rng.fill_bytes(&mut buffer);
}