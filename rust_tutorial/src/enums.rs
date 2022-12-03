fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Play Day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today weekend {}", today.is_weekend());
}
