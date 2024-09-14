/************** Implementing Selection Sort in rust **********/

fn selection_sort(array: &mut [i32]) {
    let mut min_index: usize;
    for i in 0..array.len()-1 {
        min_index = i;
        for j in i+1..array.len() {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            array.swap(min_index, i);
        }
    }
}

fn main() {
    let mut array = [1,4,2,8,345,123,43,32,5643,63,123,43,2,55,1,234,92];
    selection_sort(&mut array);
    println!("{:?}", array);
}


