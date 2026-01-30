use rand::seq::IteratorRandom;

const EN_CURVED_BRANDS: [&str; 47] = [
    "Ben and Jerry's",
    "Best Buy",
    "Bounty",
    "Butterfinger",
    "Cheerios",
    "Cheetos",
    "Chevy",
    "Coca-Cola",
    "Converse",
    "Dannon",
    "Dasani",
    "Dell",
    "Disney",
    "Doritos",
    "Dove",
    "Glade",
    "Google",
    "JCPenney",
    "Jeep",
    "John Deere",
    "Old Navy",
    "Old Spice",
    "Oreo",
    "Pampers",
    "PayPal",
    "Pepsi",
    "Playstation",
    "Pop-Tarts",
    "Porsche",
    "Pringles",
    "Purina",
    "Rolls-Royce",
    "Ruffles",
    "Samsung",
    "Sears",
    "Smart Water",
    "Sony",
    "Southwest",
    "Sprint",
    "Sprite",
    "Starbucks",
    "State Farm",
    "Subaru",
    "Subway",
    "Sun Chips",
    "Under Armour",
    "UPS",
];

const EN_STRAIGHT_BRANDS: [&str; 47] = [
    "Adidas",
    "Aflac",
    "Allstate",
    "Amazon",
    "American Eagle",
    "Android",
    "Apple",
    "Eclipse",
    "ESPN",
    "FedEx",
    "Ford",
    "Fox",
    "Heinz",
    "Hershey's",
    "Hollister",
    "Honda",
    "Hot Pockets",
    "Kia",
    "Lego",
    "Levi's",
    "Lipton",
    "McDonald's",
    "Milky Way",
    "Monster",
    "Motorola",
    "Mountain Dew",
    "Nabisco",
    "Nesquik",
    "Nestlé",
    "Nike",
    "Nintendo",
    "Nissan",
    "Nokia",
    "Target",
    "Tesla",
    "Tide",
    "Tostitos",
    "Toyota",
    "Twix",
    "Velveeta",
    "Verizon",
    "Victoria's Secret",
    "Visa",
    "Vitamin Water",
    "Walmart",
    "Xbox",
    "Yahoo",
];

const CURVED: [char; 11] = ['B', 'C', 'D', 'G', 'J', 'O', 'P', 'Q', 'R', 'S', 'U'];

fn print_usage() -> ! {
    eprintln!(
        "Usage:
cargo run <arg>
    arg: The company you want, should start with uppercase"
    );
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let Some(word) = args.next() else {
        print_usage();
    };
    if args.next().is_some() {
        print_usage();
    };
    if word.len() < 2 {
        print_usage();
    }
    if let Some(c) = word.chars().next() {
        if !c.is_ascii_uppercase() {
            print_usage();
        }
        println!("{word}");
        let mut rng = rand::rng();
        if CURVED.contains(&c.to_ascii_uppercase()) {
            EN_STRAIGHT_BRANDS
                .iter()
                .choose_multiple(&mut rng, 7)
                .iter()
                .for_each(|brand| println!("{}", brand));
        } else {
            EN_CURVED_BRANDS
                .iter()
                .choose_multiple(&mut rng, 7)
                .iter()
                .for_each(|brand| println!("{}", brand));
        }
    } else {
        print_usage();
    }
    args.for_each(|e| println!("{}", e));
}
