use solar_time::{Date, Time, Timestamp};



fn main() {
    // Test the difference between solar time and local time at various places of the world
    // Print the results in a table format with colors showing the most variance in red and least in green (gradient)
    // The table should contain the following columns:
    // - Country/City
    // - Local Time (LST)
    // - Solar Time (SST)
    // - Difference (SST - LST)
    // - Timezone
    
    let locations = vec![
        ("Calicut, India", 11.2588, 75.7804),
        ("Delhi, India", 28.6139, 77.2090),
        ("Mumbai, India", 19.0760, 72.8777),
        ("Chennai, India", 13.0827, 80.2707),
        ("Kolkata, India", 22.5051, 87.3619),
        ("Bangalore, India", 12.9716, 77.5946),
        ("Hyderabad, India", 17.3850, 78.4867),
        ("Ahmedabad, India", 23.0225, 72.5714),
        ("Assam, India", 26.2006, 92.9376),
        ("New York, USA", 40.7128, -74.0060),
        ("London, UK", 51.5074, -0.1278),
        ("Sydney, Australia", -33.8688, 151.2153),
        ("Tokyo, Japan", 35.6895, 139.6917),
        ("Rio de Janeiro, Brazil", -22.9068, -43.1729),
        ("Cape Town, South Africa", -33.9249, 18.4241),
        ("Moscow, Russia", 55.7558, 37.6173),
        ("Dubai, UAE", 25.2048, 55.2708),
        ("Anchorage, USA", 61.2181, -149.9003),
        ("Paris, France", 48.8566, 2.3522),
        ("Cairo, Egypt", 30.0444, 31.2357),
        ("Beijing, China", 39.9042, 116.4074),
        ("Mexico City, Mexico", 19.4326, -99.1332),
        ("Buenos Aires, Argentina", -34.6037, -58.3816),
        ("Cape Verde", 16.0021, -24.0132),
        ("Reykjavik, Iceland", 64.1281, -21.9562),
        ("Nairobi, Kenya", -1.2864, 36.8172),
        ("Washington D.C., USA", 38.9072, -77.0369),
        ("Addis Ababa, Ethiopia", 9.0240, 38.7469),
    ];

    let mut differences = Vec::new();
    let mut results = Vec::new();

    for location in &locations {
        let timestamp = Timestamp::new(location.1, location.2, Date { year: 2024, month: 9, day: 11 }, Time { hour: 12, minute: 0, second: 0, millisecond: 0 });
        let local_time = timestamp.time_of_day();
        let solar_time = Timestamp::calculate_solar_time(location.1, location.2, &timestamp.date(), local_time);
        let difference = (solar_time - local_time) as f64 / 3600000.0; // Convert to hours
        let timezone = Timestamp::get_timezone_offset(location.2, &timestamp.date());
        
        differences.push(difference.abs());
        results.push((location.0, local_time, solar_time, difference, timezone));
    }

    let max_difference = differences.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_difference = differences.iter().cloned().fold(f64::INFINITY, f64::min);

    println!("| {:<25} | {:<12} | {:<12} | {:<12} | {:<8} |", "Country/City", "Local Time", "Solar Time", "Difference", "Timezone");
    println!("{:-<78}", "");

    for (idx, (location, local_time, solar_time, difference, timezone)) in results.iter().enumerate() {
        let gradient = (differences[idx] - min_difference) / (max_difference - min_difference);
        let red = (255.0 * gradient) as u8;
        let green = (255.0 * (1.0 - gradient)) as u8;
        let color = format!("\x1b[38;2;{};{};0m", red, green);
        
        println!("{}| {:<25} | {:02}:{:02}:{:02} | {:02}:{:02}:{:02} | {:+6.2} hours | {:+5.1}    |\x1b[0m", 
            color,
            location,
            (local_time / 3600000) % 24,
            (local_time / 60000) % 60,
            (local_time / 1000) % 60,
            (solar_time / 3600000) % 24,
            (solar_time / 60000) % 60,
            (solar_time / 1000) % 60,
            difference,
            timezone
        );
    }
}