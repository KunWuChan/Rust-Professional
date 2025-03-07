/*
    Find Duplicates in Array
    Given an array, find all the duplicate elements and return them. 
    You need to solve the problem with O(1) space complexity (i.e., without using extra arrays or hash tables).

    Implement the function `find_duplicates(nums: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the duplicate elements in the array.
    
    Hint: You can modify the input array in place to track duplicates.
*/

use std::fmt::{self, Display, Formatter};

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    // 如果数组为空或只有一个元素，无重复
    if nums.len() <= 1 {
        return Vec::new();
    }

    // 原地排序
    nums.sort_unstable();

    // 收集重复元素
    let mut duplicates = Vec::new();
    for i in 1..nums.len() {
        // 如果当前元素与前一个相等，且尚未加入 duplicates
        if nums[i] == nums[i - 1] && (duplicates.is_empty() || *duplicates.last().unwrap() != nums[i]) {
            duplicates.push(nums[i]);
        }
    }

    duplicates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 2, 3];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![4, 5, 6, 7, 5, 4];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_find_duplicates_5() {
        let nums = vec![10, 9, 8, 7, 6, 7, 8];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![7, 8]);
    }
}
