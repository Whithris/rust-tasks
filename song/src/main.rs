fn main() {
    let days = ["second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let new = [
        "Two turtle doves,", 
        "Three French hens,", 
        "Four calling birds,", 
        "Five golden rings,", 
        "Six geese a-laying,", 
        "Seven swans a-swimming,", 
        "Eight maids a-milking,", 
        "Nine ladies dancing,", 
        "Ten lords a-leaping,", 
        "Eleven pipers piping,", 
        "Twelve drummers drumming,"];
    let mut text: Vec<&str> = Vec::new();
    let mut ends = Vec::new();
    ends.push(String::from("A partridge in a pear tree."));
    for _ in 0..9 {
        ends.push(String::from("And a partridge in a pear tree."));
    }
    ends.push(String::from("And a partridge in a pear tree!"));
    let mut index = 0;
    while index < 12 {
        println!("On the {} day of Christmas", days[index]);
        println!("my true love sent to me");
        text.push(new[index]);
        for element in text.iter().rev(){
            println!("{}", element);
        }
        println!("{}\n", ends[index]);
        
        index += 1;
    }
}
