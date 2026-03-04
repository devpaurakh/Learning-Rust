

// Returns a DateTime one billion seconds after start.
// pub fn after(start: DateTime) -> DateTime {
//     start + Duration::seconds(1_000_000_000)
// }

// fn main(){

//     let birth_date = DateTime::new(time::Date::from_calendar_date(2002, time::Month::July, 17).unwrap(), 
//     time::Time::from_hms(4, 0, 0).unwrap());

//     let new_date = after(birth_date);

//     println!("This is the new data after billion second: {}", new_date);
    
// }


//Question number 2 here I want reverse the string.

// pub fn reverse(input: &str) -> String {

//    let reversed_string = input.chars().rev().collect();

//    println!("your text is {} and reversed string is {}", input,reversed_string);

//    reversed_string
// }

// fn main(){

//     let straight_string = String::from("anupama");

//     reverse(&straight_string);

// }


//Number third quest

pub fn is_leap_year(year: u64) -> bool {
     (year%4 == 0 && year%100 !=0) || (year%400 == 0)
}

fn  main() {
    println!("This is leap year: {}",is_leap_year(2001));
    
}