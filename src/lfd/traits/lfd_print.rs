pub const INDENT_SIZE: usize = 2;

pub trait LfdPrint {
    fn lfd_print(&self, indent: usize);
}
