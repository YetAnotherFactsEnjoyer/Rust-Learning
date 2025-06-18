mod sort_arr;
use sort_arr::sort_arr;

fn main() {
    let mut nbs = [2, 1, 4, 2, 5];
    let size = nbs.len();

    sort_arr(&mut nbs, size);
    println!("Sorted {:?}", nbs);
}
