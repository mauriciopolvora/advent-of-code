use std::fs;

fn main() {


    let contents1 =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    //block[1] is seeds
    //block[2] is seed-to-soil
    //block[3] is soil-to-fertilizer
    //block[4] is fertilizer-to-water
    //block[5] is water-to-light
    //block[6] is light-to-temperature
    //block[7] is temperature-to-humidity
    //block[8] is humidity-to-location
    let blocks: Vec<&str> = contents1.split("\n\n").collect();

    //process seed-soil map
    let seed_soil_range = process_map(blocks[1]);

    //process soil-fertilizer map
    let soil_fertilizer_range = process_map(blocks[2]);

    //process fertilizer-water map
    let fertilizer_water_range = process_map(blocks[3]);

    //process water-light map
    let water_light_range = process_map(blocks[4]);

    //process light-temperature map
    let light_temperature_range = process_map(blocks[5]);

    //process temperature-humidity map
    let temperature_humidity_range = process_map(blocks[6]);

    //process humidity-location map
    let humidity_location_range = process_map(blocks[7]);

    //create empty location vector
    let mut location_vec: Vec<i64> = Vec::new();


    let seeds: Vec<i64> = blocks[0]
    .split_whitespace().skip(1)
    .map(|s| s.parse().unwrap())
    .collect();

    println!("seeds: {:?}", seeds);

    for seed in seeds {

        print!("Seed {}", seed);

        let soil = get_next_value(&seed_soil_range, &seed);
        print!(", soil {}", soil);

        let fertilizer = get_next_value(&soil_fertilizer_range, &soil);
        print!(", fertilizer {}", fertilizer);

        let water = get_next_value(&fertilizer_water_range, &fertilizer);
        print!(", water {}", water);

        let light = get_next_value(&water_light_range, &water);
        print!(", light {}", light);

        let temperature = get_next_value(&light_temperature_range, &light);
        print!(", temperature {}", temperature);

        let humidity = get_next_value(&temperature_humidity_range, &temperature);
        print!(", humidity {}", humidity);

        let location = get_next_value(&humidity_location_range, &humidity);
        location_vec.push(location);
        println!(", location {}", location);
        
    }    

    let min = get_lowest_value(location_vec);
    println!("lowest location: {}", min);

}


fn process_map(input: &str) -> Vec<(i64, i64, i64)> {
    let mut ranges: Vec<(i64, i64, i64)> = Vec::new();

    for line in input.lines().skip(1) {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        ranges.push((numbers[0], numbers[1], numbers[2]));
    }

    println!("ranges: {:?}", ranges);
    ranges
}

fn get_next_value(ranges: &Vec<(i64, i64, i64)>, value: &i64) -> i64 {

    for (destination, source, range) in ranges {

        
        let range = *source..(*source + (*range - 1));
        let difference = source - destination;

        if range.contains(value) {
            if source > destination {

                return value + difference;
            } else {
                return value - difference;
            }
        }

    }

    *value
}

fn get_lowest_value(vec: Vec<i64>) -> i64 {
    *vec.iter().min().unwrap()
}