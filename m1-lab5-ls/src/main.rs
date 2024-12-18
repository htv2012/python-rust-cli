mod cli;
mod filesys;
fn main() {
    let target = cli::get_target();
    filesys::list_files(&target);
}
