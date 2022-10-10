mod backup;

// let args: Vec<_> = env::args().collect();
fn main() {
    let backup = backup::backup::Backup::new("./config.yaml");
    backup.print();
    backup.execute()
}
