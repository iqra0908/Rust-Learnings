// Given an integer array nums and an integer k, return the kth largest element in the array.
// Note that it is the kth largest element in the sorted order, not the kth distinct element.
//You must solve it in O(n) time complexity.
// Example 1:
// Input: nums = [3,2,1,5,6,4], k = 2
// Output: 5

// Example 2:
// Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
// Output: 4

// Constraints:
// 1 <= k <= nums.length <= 104
// -104 <= nums[i] <= 104

// Solution 1: Sort
// Time complexity: O(nlogn)
// Space complexity: O(1)

pub fn find_kth_largest_sort(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums[nums.len() - k as usize]
}

// Solution 2: Min Heap
// Time complexity: O(nlogk)
// Space complexity: O(k)

pub fn find_kth_largest_minheap(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::with_capacity(k as usize);
    for num in nums {
        heap.push(std::cmp::Reverse(num));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.pop().unwrap().0
}


// Solution 3: Max Heap
// Time complexity: O(nlogn)
// Space complexity: O(n)
pub fn find_kth_largest_maxheap(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for num in nums {
        heap.push(num);
    }
    for _ in 0..k - 1 {
        heap.pop();
    }
    heap.pop().unwrap()
}


// Solution 4: Quick Select
// Time complexity: O(n)
// Space complexity: O(1)
pub fn find_kth_largest_quickselect(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let k = k as usize;
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let pivot = partition(&mut nums, left, right);
        if pivot == k - 1 {
            return nums[pivot];
        } else if pivot < k - 1 {
            left = pivot + 1;
        } else {
            right = pivot - 1;
        }
    }
    unreachable!()
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut i = left;
    for j in left..right {
        if nums[j] > pivot {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, right);
    i
}


// run rust file and print output
fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    println!("{}", find_kth_largest_sort(nums.clone(), k));
    println!("{}", find_kth_largest_minheap(nums.clone(), k));
    println!("{}", find_kth_largest_maxheap(nums.clone(), k));
    println!("{}", find_kth_largest_quickselect(nums.clone(), k));
}

