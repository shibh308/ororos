
use util::abort_str;

const PAGE_NUM: usize = 0x10;      // 0x100
const PAGE_SIZE: usize = 0x10000;  // 0x1000
// ページ管理の構造体
pub struct Paging {
    pages: [usize; PAGE_NUM],
}

impl Paging {
    pub fn new() -> Paging {
        Paging {
            pages: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
    pub fn assign(&mut self, pid: usize) -> usize {
        // pid:0 と page_id:0 は使わない
        for i in 1..PAGE_NUM {
            if self.pages[i] == 0 {
                self.pages[i] = pid;
                return i;
            }
        }
        abort_str("page assign error\n");
        0
    }
    pub fn unassign(&mut self, pid: usize) {
        for i in 1..PAGE_NUM {
            if self.pages[i] == pid {
                self.pages[i] = 0;
                return;
            }
        }
        abort_str("page unassign error\n");
    }
}