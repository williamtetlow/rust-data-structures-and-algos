use rand::Rng;

use crate::sorting::{bubble_sort, merge_sort, quick_sort, selection_sort};

mod sorting;

fn main() {
    let mut rng = rand::thread_rng();
    let test_vec: Vec<usize> = (0..100).map(|_| rng.gen_range(0..100)).collect();

    println!("running sort algorithms\ninput: {:?}", &test_vec);

    println!("-- bubble sort --");
    let bubble_sorted = bubble_sort(test_vec.clone());
    println!("{:?}", bubble_sorted);

    println!("-- merge sort --");
    let merge_sorted = merge_sort(test_vec.clone());
    println!("{:?}", merge_sorted);

    println!("-- quick sort --");
    let quick_sorted = quick_sort(test_vec.clone());
    println!("{:?}", quick_sorted);

    println!("-- selection sort --");
    let selection_sorted = selection_sort(test_vec);
    println!("{:?}", selection_sorted);
}
