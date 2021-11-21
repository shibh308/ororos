
use process::Process;
use util::{abort_str, write_char};

const PAGE_NUM: usize = 0x10;      // 0x100
// ページ管理の構造体
pub struct Paging {
    pages: [Option<Process>; PAGE_NUM],
}

impl Paging {
    pub fn new() -> Paging {
        Paging {
            pages: [None; PAGE_NUM],
        }
    }
    pub fn assign(&mut self, fn_ptr: fn()) -> usize {
        for i in 1..PAGE_NUM {
            if self.pages[i].is_none() {
                self.pages[i] = Some(Process::new(i, fn_ptr));
                return i;
            }
        }
        abort_str("page assign error\n");
        0
    }
    pub fn run(&mut self, page_id: usize) {
        if let Some(proc) = &mut self.pages[page_id] {
            proc.run();
        }
    }
    pub fn unassign(&mut self, page_id: usize) {
        self.pages[page_id] = None;
    }
    pub fn exists(&mut self, idx: usize) -> bool {
        self.pages[idx].is_some()
    }
    pub fn get_next_process(&mut self, prev: usize) -> usize {
        for i in 1..PAGE_NUM {
            if i != prev && self.exists(i) {
                write_char('a');
                write_char(i as u8 as char);
                return i;
            }
        }
        if self.pages[prev].is_some() {
            prev
        } else {
            0
        }
    }
    pub fn copy_from_csr(&mut self, page_id: usize) {
        if let Some(proc) = &mut self.pages[page_id] {
            proc.copy_from_csr();
        }
    }
    pub fn copy_to_csr(&mut self, page_id: usize) {
        if let Some(proc) = &mut self.pages[page_id] {
            proc.copy_to_csr();
        }
    }
}