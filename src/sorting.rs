use std::collections::VecDeque;

pub fn bubble_sort(mut input: Vec<usize>) -> Vec<usize> {
    let mut iterations = 0;

    for i in 0..input.len() {
        iterations += 1;
        for j in 0..input.len() - 1 - i {
            iterations += 1;
            if input[j] > input[j + 1] {
                input.swap(j, j + 1);
            }
        }
    }

    println!("iterations {}", iterations);
    input
}

pub fn merge_sort(input: Vec<usize>) -> Vec<usize> {
    let mut iterations = 0;
    let mut divided: VecDeque<VecDeque<usize>> =
        input.into_iter().map(|x| VecDeque::from([x])).collect();

    loop {
        iterations += 1;

        let lhs = divided.pop_front();
        let rhs = divided.pop_front();

        match (lhs, rhs) {
            (None, Some(sorted)) | (Some(sorted), None) => {
                println!("iterations {}", iterations);
                return sorted.into();
            }
            (None, None) => return vec![],
            (Some(mut lhs), Some(mut rhs)) => {
                let mut sorted = VecDeque::with_capacity(lhs.len() + rhs.len());

                'inner: while !lhs.is_empty() || !rhs.is_empty() {
                    iterations += 1;

                    match (lhs.front(), rhs.front()) {
                        (None, None) => break 'inner,
                        (Some(_), None) => {
                            sorted.append(&mut lhs);
                            break 'inner;
                        }
                        (None, Some(_)) => {
                            sorted.append(&mut rhs);
                            break 'inner;
                        }
                        (Some(l), Some(r)) => {
                            if l < r {
                                sorted.push_back(lhs.pop_front().expect("element exists"));
                            } else {
                                sorted.push_back(rhs.pop_front().expect("element exists"));
                            }
                        }
                    }
                }

                divided.push_back(sorted);
            }
        }
    }
}

pub fn quick_sort(mut input: Vec<usize>) -> Vec<usize> {
    let mut iterations = 0;
    if input.len() < 2 {
        println!("iterations {}", iterations);
        return input;
    }

    let mut partitions = vec![(0, input.len() - 1)];

    while let Some(partition) = partitions.pop() {
        iterations += 1;
        let (start, end) = partition;
        let mut cur_index = start;
        let mut pivot = end;

        while cur_index <= pivot {
            iterations += 1;
            if input[cur_index] > input[pivot] {
                if pivot - cur_index > 1 {
                    input.swap(pivot, pivot - 1);
                }
                input.swap(cur_index, pivot);
                pivot -= 1;
            } else {
                cur_index += 1;
            }
        }

        if pivot > start + 1 && pivot - start + 1 > 2 {
            partitions.push((start, pivot - 1));
        }

        if pivot < end - 1 && end - pivot + 1 > 2 {
            partitions.push((pivot + 1, end));
        }
    }

    println!("iterations {}", iterations);
    input
}

pub fn selection_sort(mut input: Vec<usize>) -> Vec<usize> {
    let mut iterations = 0;
    for i in 0..input.len() {
        iterations += 1;
        let mut smallest = i;
        for j in i..input.len() {
            iterations += 1;
            if input[j] < input[smallest] {
                smallest = j;
            }
        }

        if i != smallest {
            input.swap(i, smallest)
        }
    }

    println!("iterations {}", iterations);
    input
}
