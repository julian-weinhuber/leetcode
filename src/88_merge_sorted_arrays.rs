// src/88_merge_sorted_arrays.rs

use std::i32;

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m_i32: i32 = m - 1;
    let mut n_i32: i32 = n - 1;
    let mut index_i32: i32 = m + n - 1;

    // Special case with just 1 element in nums2
    if index_i32 == 0 && nums1.len() == 1 && n_i32 == 1 {
        nums1[0] = nums2[0];
        return;
    }

    while index_i32 >= 0 {
        let index_option: Option<usize> = index_i32.try_into().ok();
        let m_option: Option<usize> = m_i32.try_into().ok();
        let n_option: Option<usize> = n_i32.try_into().ok();

        let index_option_is_some: bool = index_option.is_some();
        let m_option_is_some: bool = m_option.is_some();
        let n_option_is_some: bool = n_option.is_some();

        // Value at index is placeholder
        if index_i32 >= m && index_option_is_some && n_option_is_some {
            // Value at nums1 is bigger than value at nums2
            if m_option_is_some && nums1[m_option.unwrap()] > nums2[n_option.unwrap()] {
                nums1[index_option.unwrap()] = nums1[m_option.unwrap()];
                nums1[m_option.unwrap()] = i32::MIN;
                m_i32 -= 1;
            } else {
                nums1[index_option.unwrap()] = nums2[n_option.unwrap()];
                n_i32 -= 1;
            }
            index_i32 -= 1;
            continue;
        }

        // Value at nums2 is biggest and nums1 is defined
        if index_option_is_some
            && m_option_is_some
            && n_option_is_some
            && nums2[n_option.unwrap()] >= nums1[index_option.unwrap()]
            && nums2[n_option.unwrap()] >= nums1[m_option.unwrap()]
        {
            nums1[index_option.unwrap()] = nums2[n_option.unwrap()];
            n_i32 -= 1;
        // Value at nums1 is biggest
        } else if m_option_is_some
            && index_option.is_some()
            && nums1[m_option.unwrap()] > nums1[index_option.unwrap()]
        {
            nums1[index_option.unwrap()] = nums1[m_option.unwrap()];
            nums1[m_option.unwrap()] = i32::MIN;
            m_i32 -= 1
        // Value at nums2 is biggest and nums1 is undefined
        } else if index_option_is_some
            && n_option_is_some
            && nums2[n_option.unwrap()] >= nums1[index_option.unwrap()]
        {
            nums1[index_option.unwrap()] = nums2[n_option.unwrap()];
            n_i32 -= 1;
        } else {
            m_i32 -= 1
        }
        index_i32 -= 1;
    }
}

fn main() {
    let mut nums1: Vec<i32> = vec![2, 0];
    let m: i32 = 1;
    let mut nums2: Vec<i32> = vec![1];
    let n: i32 = 1;

    merge(&mut nums1, m, &mut nums2, n);

    println!("nums1: {:?}", nums1);
}
