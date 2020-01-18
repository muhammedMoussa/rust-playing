// something like array but in fixed length

pub fn run() {
   let person: ( &str, &str, i8 ) = ("Muhammed", "Egypt", 24);

   println!("{} from {} and {}", person.0, person.1, person.2)
}