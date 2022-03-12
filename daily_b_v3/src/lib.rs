
pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut ret : Vec<String> = vec!["".to_string()];

    for var in s.chars(){
        if var.is_digit(10) {
            ret = ret.into_iter().map(|mut x| {x.push(var); x}).collect();
        } else {
            for i in 0..ret.len() {
                let mut temp = ret[i].clone();
                temp.push(var.to_ascii_lowercase());
                ret.push(temp);
                ret[i].push(var.to_ascii_uppercase());
            }
        }
    }
    return ret;
}

pub fn get_char(c: char) -> String {
    let res = {
        match c {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _ => "",
        }
    };
    return res.into();
}
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty(){
        return vec![];
    }

    digits.chars().fold(vec![String::from("")], 
    |acc, digit| acc.iter().flat_map(
        |x| get_char(digit).chars().map(
            |y| format!("{}{}", x, y)
            ).collect::<Vec<String>>()
        ).collect()
    )
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    if board.len() == 0 || word.len() == 0 || board[0].len() == 0 {return false;}  
    let mut used = vec![vec![false;board[0].len()];board.len()];
    let letters = word.chars().collect::<Vec<char>>();

    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] == letters[0] {
                used[x][y] = true;
                if search(&board, &mut used, x, y, &letters[1..]) {return true;}
                used[x][y] = false;
            }
        }
    }
    return false;
}

pub fn search(board: &Vec<Vec<char>>, used: &mut Vec<Vec<bool>>, x: usize, y: usize, letters: &[char]) -> bool {
    if letters.len() == 0 {return true;}

    if x > 0 && !used[x - 1][y] && board[x - 1][y] == letters[0] {
        used[x-1][y] = true;
        if search(board, used, x-1, y, &letters[1..]) {return true}
        used[x-1][y] = false;
    }
    if y > 0 && !used[x][y - 1] && board[x][y - 1] == letters[0] {
        used[x][y - 1] = true;
        if search(board, used, x, y - 1, &letters[1..]) {return true}
        used[x][y - 1] = false;
    }
    if x < board.len() - 1 && !used[x + 1][y] && board[x + 1][y] == letters[0] {
        used[x + 1][y] = true;
        if search(board, used, x + 1, y, &letters[1..]) {return true}
        used[x + 1][y] = false;
    }
    if y < board[0].len() - 1 && !used[x][y + 1] && board[x][y + 1] == letters[0] {
        used[x][y + 1] = true;
        if search(board, used, x, y + 1, &letters[1..]) {return true}
        used[x][y + 1] = false;
    }
    return false;
}

use std::cmp::max;

pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != 0 {
                sum =  max(sum, find_max(&mut grid, x, y));
            }
        }
    }
    return sum;
}

    pub fn find_max(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        println!("x: {}, y: {}", x, y);
        if grid[x][y] == 0 {
            return 0;
        }

        let current = grid[x][y];
        grid[x][y] = 0;
        
        let left = {
            if y == 0 {0}
            else {find_max(grid, x, y - 1)}
        };
        let right = {
            if y == grid[0].len() - 1 {0}
            else {find_max(grid, x, y + 1)}
        };
        let up = {
            if x == 0 {0}
            else {find_max(grid, x - 1, y)}
        };
        let down = {
            if x == grid.len() - 1 {0}
            else {find_max(grid, x + 1, y)}
        };

        grid[x][y] = current;

        let horizontal_max = max(left, right);
        let vertical_max = max(up, down);
        return max(horizontal_max, vertical_max) + current;
    }

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = Vec::<String>::new();
        gen_helper(&mut result, n, n, "".to_string());
        result
    }
    
    pub fn gen_helper(result: &mut Vec<String>, left: i32, right: i32, sublist: String) {       
        if left == 0 && right == 0 {
            result.push(sublist);
            return;
        }
        
        if left > 0 {
            gen_helper(result, left - 1, right, sublist.clone() + "(");
        }
        if right > left {
            gen_helper(result, left, right - 1, sublist.clone() + ")");
        }
    }

// find target (n) in digits (k) using 1..9 without repeating
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut comb = Vec::<i32>::new();
    let mut ret = Vec::<Vec<i32>>::new();
    
    backtrack3(n, k, &mut comb, 1, &mut ret);
    return ret;
}
    pub fn backtrack3(remain: i32, arr_lim:i32, comb: &mut Vec<i32>, start: i32, ret: &mut Vec<Vec<i32>>) {
        if remain == 0 && comb.len() == arr_lim as usize {
            ret.push(comb.to_vec());
            return;
        } else if (remain < 0) || comb.len() == arr_lim as usize{
            return;
        }

        for x in start..10 {
            comb.push(x);
            backtrack3(remain-x, arr_lim, comb, x+1, ret);
            comb.pop();
        }
    }

// find digits that add up to target from list (candidates, no repeats in list), you are able to repeat digits in result
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut comb = Vec::<i32>::new();
    let mut ret = Vec::<Vec<i32>>::new();
    
    backtrack(target, &candidates, &mut comb, 0, &mut ret);
    return ret;
}
    pub fn backtrack(remain: i32, candidates: &Vec<i32>, comb: &mut Vec<i32>, start: usize, ret: &mut Vec<Vec<i32>>) {
        if remain == 0 {
            ret.push(comb.to_vec());
            return;
        } else if remain < 0 {
            return;
        }

        for ind in start..candidates.len() {
            comb.push(candidates[ind]);
            backtrack(remain-candidates[ind], candidates, comb, ind, ret);
            comb.pop();
        }
    }

// find digits that add up to target from list (candidates, repeats in list), you are not able to repeat digits in result
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
let mut ret = Vec::<Vec<i32>>::new();
let mut comb = Vec::<i32>::new();
candidates.sort();

backtrack2(target, &candidates, &mut comb, 0, &mut ret);
return ret;
}
    pub fn backtrack2(remain: i32, candidates: &[i32], comb: &mut Vec<i32>, start: usize, ret: &mut Vec<Vec<i32>>) {
        if remain == 0 {
            ret.push(comb.to_vec());
            return;
        } else if remain < 0 {
            return;
        }
        
        for curr in start..candidates.len() {
            if curr > start && candidates[curr] == candidates[curr-1] {
                continue;
            } 
            comb.push(candidates[curr]);
            backtrack2(remain - candidates[curr], candidates, comb, curr+1, ret);
            comb.pop();
        }        
    }

pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut ret = Vec::<Vec<String>>::new();
    dfs(0, &mut ret, &mut Vec::<String>::new(), &s);
    return ret;
}
    pub fn dfs(start: usize, ret: &mut Vec<Vec<String>>, curr_list: &mut Vec<String>, s: &String) {

        if start >= s.len() {
            ret.push(curr_list.to_vec());
        }
        let str_temp = s.clone();
        for end in start..s.len() {
            if is_palindrome( &s , start, end) {
                // add current substring in the CurrentList
                println!("{}", s.get(start..end+1).unwrap().to_string());
                curr_list.push(s.get(start..end+1).unwrap().to_string().clone());
                dfs(end+1, ret, curr_list, &str_temp);
                curr_list.pop();
            }
        }
        
    }
    pub fn is_palindrome(s: &String, mut low: usize, mut high: usize) -> bool {
        while low < high {
            println!("df");
            if s.chars().nth(low).unwrap() != s.chars().nth(high).unwrap() {
                return false;
            }
            low += 1;
            high -= 1;
        }
        return true;
    }

pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();

    let mut last = people.len() -1;
    let mut first = 0;
    let mut counter = 0;

   while first <= last {
        counter += 1;
        if people[first] + people[last] <= limit{
            first += 1;
        }
        if last ==  0 {
            break;
        }
        last -= 1;
        
    }
    return counter;
}

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    if tokens.len()==0{
        return 0;
    }
    tokens.sort();
    let mut left = 0;
    let mut right = tokens.len()-1;
    let mut points = 0;
    
    while left <= right{
        if  power >= tokens[left] {
            power -= tokens[left];
            points += 1;
            left += 1;
        }else if points >= 1 && right - left > 1 {
            power += tokens[right];
            points -= 1;
            right -=1
        } else {
            break;
        }
    }
    return points;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CostInd{
    Cost(i32,usize),
}

pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    use CostInd::*;
    let mut net_cost = Vec::<CostInd>::new();
    let mut ret = 0;
    
    for (ind,x) in costs.iter().enumerate() {
        let temp = x[0] - x[1];
        net_cost.push(Cost(temp, ind));
    }
    net_cost.sort();
    
    for (ind,x) in net_cost.iter().enumerate() {
        if ind < net_cost.len()/2 {
            let Cost(_,k) = *x;
            ret += costs[k][0];
        } else {
            let Cost(_,k) = *x;
            ret += costs[k][1];
        }
    }

    return ret;
}

use std::cmp::min;
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    return helper(&cost, cost.len());
}
    pub fn helper(cost: &Vec<i32>, num: usize ) -> i32 {
        if num <= 1{
            return 0;
        }
        return min(cost[num -1] + helper(cost, num-1), cost[num-2] + helper(cost, num-2));
    }

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut list = vec![None; (amount + 1 )as usize];
    list[0] = Some(0);

    for x in 0..(amount+1) as usize {
        if let Some(step) = list[x] {
            for coin in coins.iter() {
                if let Some(var) = list.get_mut(x + *coin as usize) {
                    var.replace(var.map_or(step + 1, |n| n.min(step + 1)));
                }
            }
        }
    }

    return list[amount as usize].unwrap_or(-1)
}



#[cfg(test)]
mod tests {
    use crate::{letter_case_permutation, letter_combinations, exist, get_maximum_gold, two_city_sched_cost, coin_change};
    use crate::{generate_parenthesis, combination_sum3, combination_sum, combination_sum2};
    use crate::{partition, num_rescue_boats, bag_of_tokens_score, min_cost_climbing_stairs};

    #[test]
    fn letter_permutation() {
        assert_eq!(vec!["A1B2","a1B2","A1b2","a1b2"], letter_case_permutation("a1b2".to_string()));
    }

    #[test] 
    fn letter_comb() {
        assert_eq!(vec!["ad","ae","af","bd","be","bf","cd","ce","cf"], letter_combinations("23".to_string()));
    }

    #[test]    
    fn word_search() {
        assert_eq!(true, exist(vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ], "ABCCED".to_string()));
    }

    #[test]
    fn max_gold() {
        assert_eq!(28, get_maximum_gold(vec![
            vec![1,0,7],
            vec![2,0,6],
            vec![3,4,5],
            vec![0,3,0],
            vec![9,0,20]
        ]));
    }

    #[test]
    fn gen_parenth() {
        assert_eq!(vec![
            "((()))","(()())",
            "(())()","()(())",
            "()()()"],
            generate_parenthesis(3)
        );
    }

    #[test]
    fn comb_sum3() {
        assert_eq!(vec![vec![1,2,4]], combination_sum3(3,7));
    }

    #[test]
    fn comb_sum() {
        assert_eq!(vec![vec![2,2,3], vec![7]], combination_sum(vec![2,3,6,7],7));
    }

    #[test]
    fn comb_sum2() {
        assert_eq!(vec![vec![1,1,6], vec![1,2,5],vec![1,7], vec![2,6]]
            , combination_sum2(vec![10,1,2,7,6,1,5],8));
    }

    #[test]
    fn palin(){
        assert_eq!(vec![vec!["a","a","b"], vec!["aa", "b"]], partition("aab".to_string()));
    }

    #[test]
    fn life_rafts(){
        assert_eq!(4, num_rescue_boats([5,5,5,5].to_vec(), 5));
    }

    #[test]
    fn token_bag(){
        assert_eq!(2, bag_of_tokens_score([100,200,300,400].to_vec(), 200));
    }

    #[test]
    fn cheap_flights(){
        assert_eq!(110, two_city_sched_cost([vec![10,20], vec![30,200], vec![400,50], vec![30,20]].to_vec()));
    }

    #[test]
    fn stairs(){
        assert_eq!(15, min_cost_climbing_stairs(vec![10,15,20]));
    }

    #[test]
    fn coins(){
        assert_eq!(3, coin_change(vec![1,2,5], 11));
    }
}


