use solar_time::{Date, Time, Timestamp};



// fn main() {
 
//     let locations = vec![
//         ("Calicut, India", 11.2588, 75.7804),
//         ("Delhi, India", 28.6139, 77.2090),
//         ("Mumbai, India", 19.0760, 72.8777),
//         ("Chennai, India", 13.0827, 80.2707),
//         ("Kolkata, India", 22.5051, 87.3619),
//         ("Bangalore, India", 12.9716, 77.5946),
//         ("Hyderabad, India", 17.3850, 78.4867),
//         ("Ahmedabad, India", 23.0225, 72.5714),
//         ("Assam, India", 26.2006, 92.9376),
//         ("New York, USA", 40.7128, -74.0060),
//         ("London, UK", 51.5074, -0.1278),
//         ("Sydney, Australia", -33.8688, 151.2153),
//         ("Tokyo, Japan", 35.6895, 139.6917),
//         ("Rio de Janeiro, Brazil", -22.9068, -43.1729),
//         ("Cape Town, South Africa", -33.9249, 18.4241),
//         ("Moscow, Russia", 55.7558, 37.6173),
//         ("Dubai, UAE", 25.2048, 55.2708),
//         ("Anchorage, USA", 61.2181, -149.9003),
//         ("Paris, France", 48.8566, 2.3522),
//         ("Cairo, Egypt", 30.0444, 31.2357),
//         ("Beijing, China", 39.9042, 116.4074),
//         ("Mexico City, Mexico", 19.4326, -99.1332),
//         ("Buenos Aires, Argentina", -34.6037, -58.3816),
//         ("Cape Verde", 16.0021, -24.0132),
//         ("Reykjavik, Iceland", 64.1281, -21.9562),
//         ("Nairobi, Kenya", -1.2864, 36.8172),
//         ("Washington D.C., USA", 38.9072, -77.0369),
//         ("Addis Ababa, Ethiopia", 9.0240, 38.7469),
//         ("Auckland, New Zealand", -36.8485, 174.7633),
//         ("Vancouver, Canada", 49.2827, -123.1207),
//         ("Stockholm, Sweden", 59.3293, 18.0686),
//         ("Singapore", 1.3521, 103.8198),
//         ("Istanbul, Turkey", 41.0082, 28.9784),
//         ("Bangkok, Thailand", 13.7563, 100.5018),
//         ("Lima, Peru", -12.0464, -77.0428),
//         ("Helsinki, Finland", 60.1699, 24.9384),
//         ("Seoul, South Korea", 37.5665, 126.9780),
//         ("Lisbon, Portugal", 38.7223, -9.1393),
//         ("Honolulu, USA", 21.3069, -157.8583),
//         ("Kathmandu, Nepal", 27.7172, 85.3240),
//         ("Quito, Ecuador", -0.1807, -78.4678),
//         ("Oslo, Norway", 59.9139, 10.7522),
//         ("Manila, Philippines", 14.5995, 120.9842),
//         ("Casablanca, Morocco", 33.5731, -7.5898),
//         ("Havana, Cuba", 23.1136, -82.3666),
//         ("Reykjavik, Iceland", 64.1265, -21.8174),
//         ("Ulaanbaatar, Mongolia", 47.8864, 106.9057),

//     ];

//     let mut differences = Vec::new();
//     let mut results = Vec::new();

//     for location in &locations {
//         let timestamp = Timestamp::new(location.1, location.2, Date { year: 2024, month: 9, day: 11 }, Time { hour: 12, minute: 0, second: 0, millisecond: 0 });
//         let local_time = timestamp.time_of_day();
//         let solar_time = Timestamp::calculate_solar_time(location.1, location.2, &timestamp.date(), local_time);
//         let difference = (solar_time - local_time) as f64 / 3600000.0; // Convert to hours
//         let timezone = Timestamp::get_timezone_offset(location.2, &timestamp.date());
        
//         differences.push(difference.abs());
//         results.push((location.0, local_time, solar_time, difference, timezone));
//     }

//     let max_difference = differences.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
//     let min_difference = differences.iter().cloned().fold(f64::INFINITY, f64::min);

//     println!("| {:<25} | {:<8} | {:<8} | {:<8} | {:<8} |", "Country/City", "LOCAL", "SOLAR", "DIFF", "TZ");
//     println!("{:-<78}", "");

//     for (idx, (location, local_time, solar_time, difference, timezone)) in results.iter().enumerate() {
//         let gradient = (differences[idx] - min_difference) / (max_difference - min_difference);
//         let red = (255.0 * gradient) as u8;
//         let green = (255.0 * (1.0 - gradient)) as u8;
//         let color = format!("\x1b[38;2;{};{};0m", red, green);
        
//         println!("{}| {:<25} | {:02}:{:02}:{:02} | {:02}:{:02}:{:02} | {:>+8.2} | {:>8.1} |\x1b[0m",
//             color,
//             location,
//             (local_time / 3600000) % 24,
//             (local_time / 60000) % 60,
//             (local_time / 1000) % 60,
//             (solar_time / 3600000) % 24,
//             (solar_time / 60000) % 60,
//             (solar_time / 1000) % 60,
//             difference,
//             timezone
//         );
//     }
// }




use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Sexagesimal {
    degrees: i32,
    minutes: i32,
    seconds: f64,
}

impl Sexagesimal {
    fn new(degrees: impl Into<i32>, minutes: impl Into<i32>, seconds: impl Into<f64>) -> Self {
        let mut s = Sexagesimal { degrees: degrees.into(), minutes: minutes.into(), seconds: seconds.into() };
        s.normalize();
        s
    }

    fn from_degrees(deg: f64) -> Self {
        let mut degrees = deg.trunc() as i32;
        let mut minutes = ((deg - degrees as f64) * 60.0).trunc() as i32;
        let mut seconds = (deg - degrees as f64 - minutes as f64 / 60.0) * 3600.0;
        
        seconds = (seconds * 10.0).round() / 10.0;
        
        if seconds >= 60.0 {
            minutes += 1;
            seconds -= 60.0;
        }
        if minutes >= 60 {
            degrees += 1;
            minutes -= 60;
        }
        
        Sexagesimal::new(degrees, minutes, seconds)
    }

    fn to_degrees(&self) -> f64 {
        self.degrees as f64 + self.minutes as f64 / 60.0 + self.seconds / 3600.0
    }

    fn normalize(&mut self) {
        let total_seconds = self.degrees as f64 * 3600.0 + self.minutes as f64 * 60.0 + self.seconds;
        let sign = total_seconds.signum();
        let abs_total_seconds = total_seconds.abs();

        self.degrees = (abs_total_seconds / 3600.0).trunc() as i32 * sign as i32;
        let remaining = abs_total_seconds - self.degrees.abs() as f64 * 3600.0;
        self.minutes = (remaining / 60.0).trunc() as i32;
        self.seconds = (remaining - self.minutes as f64 * 60.0).round() / 10.0 * 10.0;

        if self.seconds >= 60.0 {
            self.minutes += 1;
            self.seconds -= 60.0;
        }
        if self.minutes >= 60 {
            self.degrees += if self.degrees >= 0 { 1 } else { -1 };
            self.minutes -= 60;
        }

        self.degrees = ((self.degrees % 360) + 360) % 360;
    }

    fn normalize_latitude(&mut self) {
        self.normalize();
        if self.degrees > 180 {
            self.degrees = 360 - self.degrees;
            self.minutes = -self.minutes;
            self.seconds = -self.seconds;
        }
        if self.degrees > 90 {
            self.degrees = 180 - self.degrees;
            self.minutes = -self.minutes;
            self.seconds = -self.seconds;
        }
    }
}

impl std::ops::Add for Sexagesimal {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let total_seconds = (self.degrees + other.degrees) as f64 * 3600.0 +
                            (self.minutes + other.minutes) as f64 * 60.0 +
                            self.seconds + other.seconds;
        Sexagesimal::from_degrees(total_seconds / 3600.0)
    }
}

impl std::ops::Sub for Sexagesimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let total_seconds = (self.degrees - other.degrees) as f64 * 3600.0 +
                            (self.minutes - other.minutes) as f64 * 60.0 +
                            self.seconds - other.seconds;
        Sexagesimal::from_degrees(total_seconds / 3600.0)
    }
}

impl std::ops::Mul<f64> for Sexagesimal {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        let total_seconds = (self.degrees as f64 * 3600.0 + self.minutes as f64 * 60.0 + self.seconds) * scalar;
        Sexagesimal::from_degrees(total_seconds / 3600.0)
    }
}

const SOLAR_YEAR: Sexagesimal = Sexagesimal { degrees: 365, minutes: 15, seconds: 31.5 };
const SIDEREAL_YEAR: Sexagesimal = Sexagesimal { degrees: 365, minutes: 6, seconds: 12.3 };
const SYNODIC_MONTH: Sexagesimal = Sexagesimal { degrees: 29, minutes: 31, seconds: 50.0 };
const SIDEREAL_MONTH: Sexagesimal = Sexagesimal { degrees: 27, minutes: 19, seconds: 18.0 };
const ANOMALISTIC_MONTH: Sexagesimal = Sexagesimal { degrees: 27, minutes: 33, seconds: 16.5 };
const NODICAL_MONTH: Sexagesimal = Sexagesimal { degrees: 27, minutes: 12, seconds: 43.8 };

const REVOLUTIONS_PER_YUGA: [i64; 9] = [
    4320000, // Sun
    57753336, // Moon
    488219, // Moon's apogee
    2296832, // Mars
    17937060, // Mercury
    364220, // Jupiter
    7022388, // Venus
    146568, // Saturn
    -232226, // Moon's ascending node (Rahu)
];

struct Graha {
    name: &'static str,
    mean_longitude: Sexagesimal,
    true_longitude: Sexagesimal,
    latitude: Sexagesimal,
}

fn sexagesimal_sin(angle: Sexagesimal) -> f64 {
    (angle.to_degrees() * PI / 180.0).sin()
}

fn sexagesimal_cos(angle: Sexagesimal) -> f64 {
    (angle.to_degrees() * PI / 180.0).cos()
}

fn calc_julian_day(year: i32, month: u32, day: u32) -> f64 {
    let y = year as f64;
    let m = month as f64;
    let d = day as f64;
    
    let y = if m <= 2.0 { y - 1.0 } else { y };
    let m = if m <= 2.0 { m + 12.0 } else { m };

    let b = if y > 1582.0 || (y == 1582.0 && m > 10.0) || (y == 1582.0 && m == 10.0 && d > 15.0) {
        2.0 - (y / 100.0).floor() + ((y / 100.0).floor() / 4.0).floor()
    } else {
        0.0
    };

    (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + d + b - 1524.5
}

fn calc_ahargana(julian_day: f64) -> f64 {
    julian_day - 588465.50000 // Kali Yuga start
}

fn calc_mean_longitude(revolutions: i64, ahargana: f64) -> Sexagesimal {
    let full_revolutions = (revolutions as f64 * ahargana / 4320000.0).trunc();
    let fraction = revolutions as f64 * ahargana / 4320000.0 - full_revolutions;
    Sexagesimal::from_degrees(fraction * 360.0)
}

fn calc_manda_equation(mean_anomaly: Sexagesimal, manda_amplitude: Sexagesimal, epicycle: Sexagesimal) -> Sexagesimal {
    let sin_anomaly = sexagesimal_sin(mean_anomaly);
    let cos_anomaly = sexagesimal_cos(mean_anomaly);
    let sin_2anomaly = sexagesimal_sin(mean_anomaly * 2.0);
    
    let equation = manda_amplitude.to_degrees() * sin_anomaly +
                   epicycle.to_degrees() * sin_2anomaly +
                   manda_amplitude.to_degrees().powi(2) / 2.0 * (2.0 * sin_anomaly * cos_anomaly - sin_anomaly);
    
    Sexagesimal::from_degrees(equation)
}

fn calc_sighra_equation(sighra_anomaly: Sexagesimal, sighra_amplitude: Sexagesimal, epicycle: Sexagesimal) -> Sexagesimal {
    let sin_anomaly = sexagesimal_sin(sighra_anomaly);
    let cos_anomaly = sexagesimal_cos(sighra_anomaly);
    let sin_2anomaly = sexagesimal_sin(sighra_anomaly * 2.0);
    
    let equation = sighra_amplitude.to_degrees() * sin_anomaly +
                   epicycle.to_degrees() * sin_2anomaly +
                   sighra_amplitude.to_degrees().powi(2) / 2.0 * (2.0 * sin_anomaly * cos_anomaly - sin_anomaly);
    
    Sexagesimal::from_degrees(equation)
}

fn calc_latitudinal_equation(argument: Sexagesimal, inclination: Sexagesimal) -> Sexagesimal {
    let sin_arg = sexagesimal_sin(argument);
    Sexagesimal::from_degrees(inclination.to_degrees() * sin_arg)
}

fn calc_true_longitude(graha_index: usize, mean_longitude: Sexagesimal, ahargana: f64) -> (Sexagesimal, Sexagesimal) {
    let apogee_longitude = calc_mean_longitude(REVOLUTIONS_PER_YUGA[2], ahargana);
    let mean_anomaly = mean_longitude - apogee_longitude;

    let (manda_amplitude, sighra_amplitude, manda_epicycle, sighra_epicycle, inclination) = match graha_index {
        0 => (Sexagesimal::new(2, 0, 0), Sexagesimal::new(0, 0, 0), Sexagesimal::new(0, 14, 0), Sexagesimal::new(0, 0, 0), Sexagesimal::new(0, 0, 0)), // Sun
        1 => (Sexagesimal::new(5, 0, 0), Sexagesimal::new(0, 0, 0), Sexagesimal::new(0, 31, 30), Sexagesimal::new(0, 0, 0), Sexagesimal::new(5, 0, 0)), // Moon
        3 => (Sexagesimal::new(1, 51, 0), Sexagesimal::new(11, 30, 0), Sexagesimal::new(0, 15, 0), Sexagesimal::new(0, 15, 0), Sexagesimal::new(1, 51, 0)), // Mars
        4 => (Sexagesimal::new(4, 0, 0), Sexagesimal::new(3, 30, 0), Sexagesimal::new(0, 15, 0), Sexagesimal::new(0, 15, 0), Sexagesimal::new(2, 0, 0)), // Mercury
        5 => (Sexagesimal::new(1, 21, 0), Sexagesimal::new(5, 0, 0), Sexagesimal::new(0, 6, 0), Sexagesimal::new(0, 6, 0), Sexagesimal::new(1, 0, 0)), // Jupiter
        6 => (Sexagesimal::new(0, 39, 0), Sexagesimal::new(2, 30, 0), Sexagesimal::new(0, 15, 0), Sexagesimal::new(0, 15, 0), Sexagesimal::new(2, 30, 0)), // Venus
        7 => (Sexagesimal::new(2, 27, 0), Sexagesimal::new(6, 30, 0), Sexagesimal::new(0, 24, 0), Sexagesimal::new(0, 24, 0), Sexagesimal::new(2, 18, 0)), // Saturn
        _ => return (mean_longitude, Sexagesimal::new(0, 0, 0)), // Rahu and Ketu
    };

    let manda_equation = calc_manda_equation(mean_anomaly, manda_amplitude, manda_epicycle);
    let mut true_longitude = mean_longitude - manda_equation;

    if graha_index > 1 && graha_index < 8 {
        let sun_mean_longitude = calc_mean_longitude(REVOLUTIONS_PER_YUGA[0], ahargana);
        let sighra_anomaly = sun_mean_longitude - true_longitude;
        let sighra_equation = calc_sighra_equation(sighra_anomaly, sighra_amplitude, sighra_epicycle);
        true_longitude = true_longitude + sighra_equation;
    }

    let latitude = calc_latitudinal_equation(true_longitude - apogee_longitude, inclination);

    (true_longitude, latitude)
}

fn calc_ayanamsa(ahargana: f64) -> Sexagesimal {
    let t = ahargana / SIDEREAL_YEAR.to_degrees();
    Sexagesimal::from_degrees(54.0 / 3600.0 * t + 50.0 / 3600.0 * t * t)
}

fn calc_moon_corrections(mean_longitude: Sexagesimal, true_longitude: Sexagesimal, ahargana: f64) -> Sexagesimal {
    let sun_mean_longitude = calc_mean_longitude(REVOLUTIONS_PER_YUGA[0], ahargana);
    let elongation = true_longitude - sun_mean_longitude;
    let anomaly = true_longitude - calc_mean_longitude(REVOLUTIONS_PER_YUGA[2], ahargana);
    
    let evection = Sexagesimal::from_degrees(1.25 * sexagesimal_sin(elongation * 2.0 - anomaly));
    let variation = Sexagesimal::from_degrees(0.58 * sexagesimal_sin(elongation * 2.0));
    let yearly_equation = Sexagesimal::from_degrees(0.19 * sexagesimal_sin(sun_mean_longitude));

    true_longitude + evection + variation + yearly_equation
}

fn get_nakshatra(longitude: Sexagesimal) -> (&'static str, Sexagesimal) {
    let nakshatras = [
        "Ashwini", "Bharani", "Krittika", "Rohini", "Mrigashira", "Ardra",
        "Punarvasu", "Pushya", "Ashlesha", "Magha", "Purva Phalguni", "Uttara Phalguni",
        "Hasta", "Chitra", "Swati", "Vishakha", "Anuradha", "Hasta", "Chitra", "Swati", "Vishakha", "Anuradha", "Jyeshtha",
        "Mula", "Purva Ashadha", "Uttara Ashadha", "Shravana", "Dhanishta", "Shatabhisha",
        "Purva Bhadrapada", "Uttara Bhadrapada", "Revati"
    ];
    let nakshatra_index = (longitude.to_degrees() / (360.0 / 27.0)) as usize % 27;
    let remainder = Sexagesimal::from_degrees(longitude.to_degrees() % (360.0 / 27.0));
    (nakshatras[nakshatra_index], remainder)
}

fn calc_tithi(sun_longitude: Sexagesimal, moon_longitude: Sexagesimal) -> (u8, Sexagesimal) {
    let tithi_angle = moon_longitude - sun_longitude;
    let tithi = (tithi_angle.to_degrees() / 12.0).floor() as u8 + 1;
    let remainder = Sexagesimal::from_degrees(tithi_angle.to_degrees() % 12.0);
    (tithi, remainder)
}

fn calc_yoga(sun_longitude: Sexagesimal, moon_longitude: Sexagesimal) -> (u8, Sexagesimal) {
    let yoga_angle = sun_longitude + moon_longitude;
    let yoga = (yoga_angle.to_degrees() / (360.0 / 27.0)).floor() as u8 + 1;
    let remainder = Sexagesimal::from_degrees(yoga_angle.to_degrees() % (360.0 / 27.0));
    (yoga, remainder)
}

fn calc_karana(tithi: u8, tithi_remainder: Sexagesimal) -> (&'static str, Sexagesimal) {
    let karanas = [
        "Bava", "Balava", "Kaulava", "Taitila", "Gara", "Vanija", "Visti",
        "Sakuni", "Chatushpada", "Naga", "Kimstughna"
    ];
    let karana_index = (((tithi - 1) * 2) as f64 + if tithi_remainder.to_degrees() >= 6.0 { 1.0 } else { 0.0 }) as usize % 60;
    let remainder = if tithi_remainder.to_degrees() >= 6.0 { 
        Sexagesimal::from_degrees(tithi_remainder.to_degrees() - 6.0)
    } else { 
        tithi_remainder 
    };
    (karanas[karana_index % 11], remainder)
}

 

 
struct TestResult {
    name: String,
    surya_siddhanta: f64,
    ephemeris: f64,
    difference: f64,
}

fn run_test(year: i32, month: u32, day: u32, ephemeris_data: &[(&str, f64, f64)]) -> Vec<TestResult> {
    let jd = calc_julian_day(year, month, day);
    let ahargana = calc_ahargana(jd);
    let ayanamsa = calc_ayanamsa(ahargana);

    let mut results = Vec::new();

    for &(name, eph_long, eph_lat) in ephemeris_data {
        let graha_index = match name {
            "Sun" => 0,
            "Moon" => 1,
            "Mars" => 3,
            "Mercury" => 4,
            "Jupiter" => 5,
            "Venus" => 6,
            "Saturn" => 7,
            _ => continue,
        };

        let mean_longitude = calc_mean_longitude(REVOLUTIONS_PER_YUGA[graha_index], ahargana);
        let (true_longitude, latitude) = calc_true_longitude(graha_index, mean_longitude, ahargana);
        let sidereal_longitude = (true_longitude - ayanamsa).to_degrees();

        // Convert ephemeris tropical longitude to sidereal
        let eph_sidereal_long = (eph_long - ayanamsa.to_degrees() + 360.0) % 360.0;

        results.push(TestResult {
            name: format!("{} Longitude", name),
            surya_siddhanta: sidereal_longitude,
            ephemeris: eph_sidereal_long,
            difference: (sidereal_longitude - eph_sidereal_long + 180.0) % 360.0 - 180.0,
        });

        if graha_index != 0 {  // Sun's latitude is always 0 in Surya Siddhanta
            results.push(TestResult {
                name: format!("{} Latitude", name),
                surya_siddhanta: latitude.to_degrees(),
                ephemeris: eph_lat,
                difference: latitude.to_degrees() - eph_lat,
            });
        }
    }

    results
}

fn print_results(results: &[TestResult]) {
    println!("{:<15} {:>12} {:>12} {:>12}", "Body", "Surya Sidd.", "Ephemeris", "Difference");
    println!("{}", "-".repeat(55));

    for result in results {
        let color_code = if result.difference.abs() > 5.0 {
            "\x1B[31m" // Red for large differences
        } else if result.difference.abs() > 1.0 {
            "\x1B[33m" // Yellow for moderate differences
        } else {
            "\x1B[32m" // Green for small differences
        };

        println!("{:<15} {:>12.2} {:>12.2} {}:{:>+11.2}\x1B[0m",
                 result.name,
                 result.surya_siddhanta,
                 result.ephemeris,
                 color_code,
                 result.difference);
    }
    println!();
}

fn main() {
    let test_dates = [
        (2024, 1, 1),
        (1991, 1, 1),
        (1960, 1, 1),
        (1800, 1, 1),
        (1700, 1, 1),
    ];

    let ephemeris_data = [
        &[
            ("Sun", 280.0389850449929, 0.0001603703000799055),
            ("Moon", 155.9919311281079, 3.5676556740465744),
            ("Mercury", 262.2816893833952, 3.0649714536534036),
            ("Venus", 242.6123169424052, 1.949814500239858),
            ("Mars", 267.3083641181128, -0.5505141358661286),
            ("Jupiter", 35.582407512394624, -1.18546044459505),
            ("Saturn", 333.2435785205807, -1.6341333892856305),
        ],
        &[
            ("Sun", 280.05525435205107, -3.3592649021162056e-5),
            ("Moon", 103.23404703701473, 1.349747216695797),
            ("Mercury", 264.31202505172394, 3.1460270479450503),
            ("Venus", 294.72179412542897, -1.2841233475731566),
            ("Mars", 57.75510629640889, 2.3383429731155023),
            ("Jupiter", 131.98229056167727, 0.6817695586857513),
            ("Saturn", 295.662864429797, -0.14304138813369693),
        ],
        &[
            ("Sun", 279.57480548544726, 0.00017442959640248781),
            ("Moon", 310.26489944833224, 3.721840449391382),
            ("Mercury", 264.95910069715757, -0.17290066079502034),
            ("Venus", 238.37393439842398, 2.3013987807253202),
            ("Mars", 260.30667789044895, -0.37034800980634547),
            ("Jupiter", 258.738499788667, 0.47318194946449144),
            ("Saturn", 279.4563323882817, 0.5435138421149404),
        ],
        &[
            ("Sun", 280.42935555107846, 1.538220225998855e-5),
            ("Moon", 348.4673822891262, -3.6399275356549903),
            ("Mercury", 268.34355592564106, 3.220376975359535),
            ("Venus", 233.62727154404564, 3.2722417290599695),
            ("Mars", 244.33583185414574, 0.1228203191542599),
            ("Jupiter", 84.52699974113312, -0.30327706273265953),
            ("Saturn", 128.79918946870973, 0.6628582279684099),
        ],
        &[
            ("Sun", 280.7151285431897, -0.00020784249373191472),
            ("Moon", 40.00280512584597, -4.092668157127676),
            ("Mercury", 299.98529465470966, -0.8295515288807939),
            ("Venus", 292.3903192992935, -1.206348892836853),
            ("Mars", 203.55692407132608, 1.675532089736721),
            ("Jupiter", 280.37589973319484, -0.05753110392522196),
            ("Saturn", 328.7460752931759, -1.5713055640509634),
        ],
    ];

    for (&(year, month, day), &ephemeris) in test_dates.iter().zip(ephemeris_data.iter()) {
        println!("Test for date: {}-{:02}-{:02}", year, month, day);
        let results = run_test(year, month, day, ephemeris);
        print_results(&results);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_dates() {
        let test_dates = [
            (2024, 1, 1),
            (1991, 1, 1),
            (1960, 1, 1),
            (1800, 1, 1),
            (1700, 1, 1),
        ];

        let ephemeris_data = [
            &[
                ("Sun", 280.0389850449929, 0.0001603703000799055),
                ("Moon", 155.9919311281079, 3.5676556740465744),
                ("Mercury", 262.2816893833952, 3.0649714536534036),
                ("Venus", 242.6123169424052, 1.949814500239858),
                ("Mars", 267.3083641181128, -0.5505141358661286),
                ("Jupiter", 35.582407512394624, -1.18546044459505),
                ("Saturn", 333.2435785205807, -1.6341333892856305),
            ],
            &[
                ("Sun", 280.05525435205107, -3.3592649021162056e-5),
                ("Moon", 103.23404703701473, 1.349747216695797),
                ("Mercury", 264.31202505172394, 3.1460270479450503),
                ("Venus", 294.72179412542897, -1.2841233475731566),
                ("Mars", 57.75510629640889, 2.3383429731155023),
                ("Jupiter", 131.98229056167727, 0.6817695586857513),
                ("Saturn", 295.662864429797, -0.14304138813369693),
            ],
            &[
                ("Sun", 279.57480548544726, 0.00017442959640248781),
                ("Moon", 310.26489944833224, 3.721840449391382),
                ("Mercury", 264.95910069715757, -0.17290066079502034),
                ("Venus", 238.37393439842398, 2.3013987807253202),
                ("Mars", 260.30667789044895, -0.37034800980634547),
                ("Jupiter", 258.738499788667, 0.47318194946449144),
                ("Saturn", 279.4563323882817, 0.5435138421149404),
            ],
            &[
                ("Sun", 280.42935555107846, 1.538220225998855e-5),
                ("Moon", 348.4673822891262, -3.6399275356549903),
                ("Mercury", 268.34355592564106, 3.220376975359535),
                ("Venus", 233.62727154404564, 3.2722417290599695),
                ("Mars", 244.33583185414574, 0.1228203191542599),
                ("Jupiter", 84.52699974113312, -0.30327706273265953),
                ("Saturn", 128.79918946870973, 0.6628582279684099),
            ],
            &[
                ("Sun", 280.7151285431897, -0.00020784249373191472),
                ("Moon", 40.00280512584597, -4.092668157127676),
                ("Mercury", 299.98529465470966, -0.8295515288807939),
                ("Venus", 292.3903192992935, -1.206348892836853),
                ("Mars", 203.55692407132608, 1.675532089736721),
                ("Jupiter", 280.37589973319484, -0.05753110392522196),
                ("Saturn", 328.7460752931759, -1.5713055640509634),
            ],
        ];

        for (&(year, month, day), &ephemeris) in test_dates.iter().zip(ephemeris_data.iter()) {
            let results = run_test(year, month, day, ephemeris);
            for result in results {
                assert!(result.difference.abs() < 10.0, 
                        "Large difference for {} on {}-{:02}-{:02}: expected {}, got {}", 
                        result.name, year, month, day, result.ephemeris, result.surya_siddhanta);
            }
        }
    }
}
