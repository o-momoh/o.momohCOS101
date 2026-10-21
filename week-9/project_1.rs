use std::io::Write;

fn main() {
    let category1 = "Lager: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n";
    let category2 = "Stout: Legend, Turbo King, Williams\n";
    let category3 = "Non-Alcoholic: Maltina, Amstel Malta, Malta Gold, Fayrouz\n";

    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("create failed");
    file.write_all("NIGERIAN BREWERIES - DRINK CATEGORIES\n"
        .as_bytes()).expect("write failed");
    file.write_all(category1.as_bytes()).expect("write failed");
    file.write_all(category2.as_bytes()).expect("write failed");
    file.write_all(category3.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}