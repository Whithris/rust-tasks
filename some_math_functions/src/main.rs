// Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка: среднее значение; медиану (значение элемента из середины списка после его сортировки); моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз; HashMap будет полезна в данном случае)
use std::collections::HashMap;

fn middle(v: &Vec<i32>) -> i32 {
    let mut sum_v: i32 = 0;
    let length_v: i32 = v.len() as i32;
    for elem in v {
        sum_v += elem;
    }
    return sum_v / length_v;
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for elem in v {
        let count = map.entry(*elem).or_insert(0);
        *count += 1;
    }
    let key_with_max_value = map.iter().max_by_key(|entry| entry.1).unwrap();

    return *key_with_max_value.0;
}

fn median(mut v: Vec<i32>) -> i32 {
    let length_v: i32 = v.len() as i32;
    v.sort();
    if length_v % 2 == 1 {
        let median_index: i32 = length_v / 2;
        return v[median_index as usize];
    }
    else {
        let index_a: i32 = length_v / 2;
        let a = v[index_a as usize];
        let index_b: i32 = length_v / 2 + 1;
        let b = v[index_b as usize];
        return (a+b)/2
    }
    
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 1, 1];
    println!("Среднее значение: {}", middle(&v));
    println!("Мода: {}", mode(&v));
    println!("Медиана: {}", median(v));

}
