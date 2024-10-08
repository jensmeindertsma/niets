use std::{collections::HashMap, env, fs, net::Ipv4Addr, str::FromStr};

use owo_colors::{AnsiColors, OwoColorize};

fn main() {
    let input = fs::read_to_string("./logs.txt").unwrap();

    let events: Vec<Event> = input
        .trim()
        .split("\n------------------------------------------------------------------------\n")
        .skip(1)
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let arguments: Vec<_> = env::args().skip(1).collect();

    assert_eq!(arguments.len(), 1);

    match arguments[0].as_str() {
        "list" => {
            let ips: HashMap<Ipv4Addr, &str> = [
                ("216.39.55.13".parse().unwrap(), "Alpha"),
                ("10.41.245.114".parse().unwrap(), "Bravo"),
                ("31.13.70.7".parse().unwrap(), "Charlie"),
                ("10.2.41.8".parse().unwrap(), "Delta"),
                ("167.86.122.137".parse().unwrap(), "Echo"),
                ("54.231.142.56".parse().unwrap(), "Foxtrot"),
                ("80.247.235.238".parse().unwrap(), "Golf"),
                ("98.136.189.41".parse().unwrap(), "Hotel"),
                ("10.41.245.255".parse().unwrap(), "India"),
                ("10.2.41.7".parse().unwrap(), "Juliett"),
                ("85.93.0.32".parse().unwrap(), "Kilo"),
                ("184.168.47.225".parse().unwrap(), "Lima"),
                ("185.21.102.206".parse().unwrap(), "Mike"),
                ("192.210.137.123".parse().unwrap(), "November"),
                ("141.0.19.127".parse().unwrap(), "Oscar"),
                ("216.58.219.46".parse().unwrap(), "Papa"),
                ("54.231.130.3".parse().unwrap(), "Quebec"),
                ("185.21.102.206".parse().unwrap(), "Romeo"),
                ("206.190.37.99".parse().unwrap(), "Sierra"),
                ("204.79.197.200".parse().unwrap(), "Tango"),
                ("185.86.77.12".parse().unwrap(), "Uniform"),
            ]
            .into_iter()
            .collect();

            dbg!(&ips);

            let colors = [
                AnsiColors::Red,
                AnsiColors::Green,
                AnsiColors::Yellow,
                AnsiColors::Blue,
                AnsiColors::Magenta,
                AnsiColors::Cyan,
                AnsiColors::White,
                AnsiColors::BrightRed,
                AnsiColors::BrightGreen,
                AnsiColors::BrightYellow,
                AnsiColors::BrightBlue,
                AnsiColors::BrightMagenta,
                AnsiColors::BrightCyan,
                AnsiColors::BrightWhite,
                AnsiColors::BrightBlack,
            ];

            let mut color_map = HashMap::new();
            let mut background_map = HashMap::new();
            let mut color_index = 0;
            let mut background_index = 0;

            for event in events {
                let src_color = color_map.entry(event.src).or_insert_with(|| {
                    let color = colors[color_index % colors.len()];
                    color_index += 1;
                    color
                });

                let src = ips.get(&event.src).unwrap().color(*src_color);

                let dest_color = color_map.entry(event.dest).or_insert_with(|| {
                    let color = colors[color_index % colors.len()];
                    color_index += 1;
                    color
                });

                let kind_background =
                    background_map.entry(event.kind.clone()).or_insert_with(|| {
                        let color = colors[background_index % colors.len()];
                        background_index += 1;
                        color
                    });

                let kind = if event.kind.contains("Likely Neutrino") {
                    event.kind[..75].to_owned()
                } else {
                    event.kind
                };

                println!(
                    "{} {:>15} -> {:15} ({}) {}",
                    event.time,
                    src,
                    ips.get(&event.dest).unwrap().color(*dest_color),
                    event.id,
                    kind.color(AnsiColors::Black).on_color(*kind_background)
                )
            }
        }

        "ips" => {
            let mut ips = HashMap::new();

            for event in events {
                ips.entry(event.src).or_insert(Vec::new()).push(event);
            }

            let mut identifiers = PhoneticAlphabetIterator::new();
            let mut identifier_table = HashMap::new();

            println!("Got {} ips", ips.len());

            for (ip, events) in ips {
                let identifier = identifier_table
                    .entry(ip)
                    .or_insert_with(|| identifiers.next().unwrap());
                println!(
                    "{}",
                    format!("- {ip} ({} - {} events)", identifier, events.len()).bold()
                );

                for event in events {
                    let src = *identifier_table
                        .entry(event.src)
                        .or_insert_with(|| identifiers.next().unwrap());

                    let dest = identifier_table
                        .entry(event.dest)
                        .or_insert_with(|| identifiers.next().unwrap());

                    println!("{} -> {} ({})", src, dest, event.kind)
                }
            }

            dbg!(&identifier_table);
        }

        "kinds" => {
            let mut kinds = HashMap::new();

            for event in events {
                kinds
                    .entry(event.kind.clone())
                    .or_insert(Vec::new())
                    .push(event);
            }

            for (kind, entry) in kinds {
                println!("--- {kind} ({})", entry.len());
                for event in entry {
                    println!("* {} -> {} [{}]", event.src, event.dest, event.id)
                }
            }
        }
        _ => panic!("unknown command"),
    }
}

struct Event {
    src: Ipv4Addr,
    dest: Ipv4Addr,
    id: String,
    kind: String,
    time: String,
}

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        assert_eq!(lines.len(), 7);
        let desc = lines[0].to_owned();
        let kind = lines[1].to_owned();
        let (src, dest) = lines[2].split_once(" -> ").unwrap();

        let parts: Vec<&str> = desc.split_whitespace().collect();

        Ok(Self {
            src: src.trim().parse().unwrap(),
            dest: dest.trim().parse().unwrap(),
            kind,
            id: parts[1][6..].to_owned(),
            time: [parts[2], parts[3]].join(" "),
        })
    }
}
struct PhoneticAlphabetIterator {
    words: Vec<&'static str>,
    index: usize,
}

impl PhoneticAlphabetIterator {
    fn new() -> Self {
        PhoneticAlphabetIterator {
            words: vec![
                "Alpha", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India",
                "Juliett", "Kilo", "Lima", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo",
                "Sierra", "Tango", "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu",
            ],
            index: 0,
        }
    }
}

impl Iterator for PhoneticAlphabetIterator {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.words.len() {
            let word = self.words[self.index];
            self.index += 1;
            Some(word)
        } else {
            None
        }
    }
}
