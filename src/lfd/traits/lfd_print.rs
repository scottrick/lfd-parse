pub const INDENT_SIZE: usize = 2;

pub trait LfdPrint {
    fn lfd_get_print_str(&self) -> String;

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }
}
