
use triangle::Triangle;

mod natural_sort;
mod triangle;
mod file_handler;


fn main() {
    //let _args: Vec<String> = env::args().collect();

    //let path = natural_sort::random_data::<Triangle>(10).unwrap_or_else(|err| {
    //    panic!("cannot generate random data: {err}");
    //});
    let path = "/home/krzysiu/db_merge_sort/data/data.hex";
    natural_sort::sort::<Triangle>(path.to_string(), 2).expect("Problem sorting");
}
