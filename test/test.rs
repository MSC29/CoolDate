// use lib::entities::anniversary::Anniversary;

fn main() {
    let date: &str = "1989-06-19T00:00:00.000Z";
    let anniversaries = lib::find_anniversaries_future(date);
    anniversaries.for_each(|a| println!(format!("{}: {}", a.name, a.date)));
}
