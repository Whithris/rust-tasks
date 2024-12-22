// from leetcode
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let length = s.len() as i32;
        let mut index = 0;
        let mut j = 0;
        let mut cur_symb = 0;
        let mut result = "_".repeat(s.len());
        let mut result_vec: Vec<char> = result.chars().collect();
        let mut sep = (num_rows*2 - 2);
        while j < num_rows {
            while index < length {
                if ((index+j) % sep == 0) ||
                    ((index % sep) - j == 0) {
                    result_vec[cur_symb] = s.chars().nth(index as usize).expect("Индекс вне диапазона");
                    cur_symb += 1;
                }
                index += 1;
            }
            index = 0;
            j += 1;
            }
        result = result_vec.iter().collect();
        return result;
    }
}
