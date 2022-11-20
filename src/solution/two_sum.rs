pub mod two_sum {
    pub fn solve(mut nums: Vec<u32>, target: u32) {
        nums.sort();
        let mut low: usize = 0;
        let mut high: usize = nums.len() - 1;

        while low < high {
            let sum: u32 = nums[low] + nums[high];
            if sum == target {
                println!("{} + {} = {}", nums[low], nums[high], target);
                break;
            } else if sum < target {
                low += 1;
            } else {
                high -= 1;
            }
        }
    }
}