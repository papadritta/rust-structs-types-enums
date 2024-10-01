#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    // Burgundy,
    // Champagne,
    Tuscany,
    Rioja,
    // NapaValley,
    Australia,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn popularity(p: WineRegions) {
    match p {
        WineRegions::Bordeaux => println!("Bordeaux - Highly Popular!"),
        _ => println!("{:?} is not popular!", p),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3: Wine = Wine {
        name: String::from("BoraBora"),
        region: WineRegions::Australia,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);
    popularity(WineRegions::Bordeaux);
    popularity(WineRegions::Australia);
}

// Modify the code to add a new variant to the `WineRegions` enum, representing a wine region of your choice. Create a new `wine3` instance that utilizes this new variant. Print information about `wine3` to verify your changes.
// Extend the program by implementing a function that takes a `WineRegions` value as input and returns a string representation of the region's popularity. For example, for the region "Bordeaux", the function could return "Highly Popular." Invoke this function with different wine regions and print the result to ensure it works correctly.
