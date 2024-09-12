/*** Implementing Bubble sort in rust *********/

fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;
    let len = arr.len();
    
    for i in 0..len{
        if !swapped {
            break;
        }
        swapped = false;
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
    }
}


fn main() {
    let mut array = [1, 4, 2, 8, 345, 123, 43, 32, 5643, 63, 123, 43, 2, 55, 1, 234, 92];
    bubble_sort(&mut array);
    println!("{:?}", array);
}
