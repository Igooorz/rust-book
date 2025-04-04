/*     
    On the first day of Christmas
    My true love sent to me
    A partridge in a pear tree

    On the second day of Christmas
    My true love sent to me
    Two turtle-doves
    And a partridge in a pear tree

    On the third day of Christmas
    My true love sent to me
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the fourth day of Christmas
    My true love sent to me
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the fifth day of Christmas
    My true love sent to me
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the sixth day of Christmas
    My true love sent to me
    Six geese a laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the seventh day of Christmas
    My true love sent to me
    Seven swans a swimming
    Six geese a-laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the eighth day of Christmas
    My true love sent to me
    Eight maids a milking
    Seven swans a swimming
    Six geese a-laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the ninth day of Christmas
    My true love sent to me
    Nine ladies dancing
    Eight maids a-milking
    Seven swans a-swimming
    Six geese a-laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the tenth day of Christmas
    My true love sent to me
    Ten lords a-leaping
    Nine ladies dancing
    Eight maids a-milking
    Seven swans a-swimming
    Six geese a-laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the eleventh day of Christmas
    My true love sent to me
    Eleven pipers piping
    Ten lords a-leaping
    Nine ladies dancing
    Eight maids a-milking
    Seven swans a-swimming
    Six geese a-laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree

    On the twelfth day of Christmas
    My true love sent to me
    Twelve drummers drumming
    Eleven pipers piping
    Ten lords a-leaping
    Nine ladies dancing
    Eight maids a-milking
    Seven swans a-swimming
    Six geese a-laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree
    And a partridge in a pear tree
*/

fn main() {
    println!("Singing...\n");
    let presents: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle-doves",
        "Three French hen",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for day in 0..=11 {
        let str_day = match day + 1 {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => ""
        };

        println!("On the {} day of Christmas", str_day);
        println!("My true love sent to me");
        let range = 0..=day;
        for i in range.rev() {
            if day != 0 && presents[i] == "A partridge in a pear tree" {
                println!("And a partridge in a pear tree");
                if day == 11 {
                    println!("And a partridge in a pear tree");
                }
                break;
            }
            
            println!("{}", presents[i]);
        }
        println!("\n");
    };
    println!("The end!");
}