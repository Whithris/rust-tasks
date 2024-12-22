// Преобразуйте строку в кодировку "поросячьей латыни" (Pig Latin). Первая согласная каждого слова перемещается в конец и к ней добавляется окончание "ay", так "first" станет "irst-fay". Слову, начинающемуся на гласную, в конец добавляется "hay" ("apple" становится "apple-hay"). Помните о деталях работы с кодировкой UTF-8!
fn main() {
    let s = String::from("first apple");
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for elem in s.split(" "){
        if vowels.iter().any(|v| elem.starts_with(*v)){
            println!("{}-hay", &elem);
        }
        else {
            println!("{}-{}ay", &elem[1..], &elem.chars().nth(0).unwrap())
        }
    }

}
