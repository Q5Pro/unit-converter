//! unit-converter: Uzunluk, ağırlık, sıcaklık, alan, hacim ve hız
//! birimleri arasında dönüşüm yapan araç.
//!
//! Kullanım:
//!     unit-converter 100 km mi
//!     unit-converter 72 F C
//!     unit-converter 5 kg lb
//!     unit-converter list
use std::collections::HashMap;
use std::env;
use std::process;

fn base_units() -> HashMap<&'static str, (&'static str, f64)> {
    let mut m = HashMap::new();
    // Uzunluk (metre cinsinden)
    m.insert("m",   ("uzunluk", 1.0));
    m.insert("km",  ("uzunluk", 1000.0));
    m.insert("cm",  ("uzunluk", 0.01));
    m.insert("mm",  ("uzunluk", 0.001));
    m.insert("mi",  ("uzunluk", 1609.344));
    m.insert("yd",  ("uzunluk", 0.9144));
    m.insert("ft",  ("uzunluk", 0.3048));
    m.insert("in",  ("uzunluk", 0.0254));
    // Ağırlık (kg cinsinden)
    m.insert("kg",  ("ağırlık", 1.0));
    m.insert("g",   ("ağırlık", 0.001));
    m.insert("mg",  ("ağırlık", 0.000001));
    m.insert("t",   ("ağırlık", 1000.0));
    m.insert("lb",  ("ağırlık", 0.453592));
    m.insert("oz",  ("ağırlık", 0.0283495));
    // Alan (m² cinsinden)
    m.insert("m2",  ("alan", 1.0));
    m.insert("km2", ("alan", 1_000_000.0));
    m.insert("ha",  ("alan", 10_000.0));
    m.insert("ac",  ("alan", 4046.86));
    m.insert("ft2", ("alan", 0.092903));
    // Hacim (litre cinsinden)
    m.insert("l",   ("hacim", 1.0));
    m.insert("ml",  ("hacim", 0.001));
    m.insert("m3",  ("hacim", 1000.0));
    m.insert("gal", ("hacim", 3.78541));
    m.insert("pt",  ("hacim", 0.473176));
    m.insert("fl",  ("hacim", 0.0295735));
    // Hız (m/s cinsinden)
    m.insert("ms",  ("hız", 1.0));
    m.insert("kph", ("hız", 0.277778));
    m.insert("mph", ("hız", 0.44704));
    m.insert("kn",  ("hız", 0.514444));
    m
}

fn convert_temp(val: f64, from: &str, to: &str) -> Option<f64> {
    let celsius = match from {
        "C" => val,
        "F" => (val - 32.0) * 5.0/9.0,
        "K" => val - 273.15,
        _ => return None,
    };
    Some(match to {
        "C" => celsius,
        "F" => celsius * 9.0/5.0 + 32.0,
        "K" => celsius + 273.15,
        _ => return None,
    })
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let units = base_units();

    if args.is_empty() || args[0] == "--help" {
        println!("Kullanım: unit-converter <değer> <kaynak_birim> <hedef_birim>");
        println!("Örnek: unit-converter 100 km mi");
        println!("       unit-converter 37 C F");
        println!("       unit-converter list");
        return;
    }
    if args[0] == "list" {
        let mut cats: HashMap<&str, Vec<&str>> = HashMap::new();
        for (k, (cat, _)) in &units { cats.entry(cat).or_default().push(k); }
        for (cat, mut ks) in cats { ks.sort(); println!("{}: {}", cat, ks.join(", ")); }
        println!("sıcaklık: C, F, K");
        return;
    }

    if args.len() < 3 {
        eprintln!("Hata: <değer> <kaynak> <hedef> belirtin."); process::exit(1);
    }
    let val: f64 = args[0].parse().unwrap_or_else(|_| { eprintln!("Geçersiz sayı."); process::exit(1); });
    let from = args[1].as_str();
    let to   = args[2].as_str();

    // Sıcaklık
    if ["C","F","K"].contains(&from) || ["C","F","K"].contains(&to) {
        match convert_temp(val, from, to) {
            Some(r) => println!("{} {} = {:.4} {}", val, from, r, to),
            None => { eprintln!("Geçersiz sıcaklık birimi."); process::exit(1); }
        }
        return;
    }

    let (fcat, fbase) = units.get(from).unwrap_or_else(|| { eprintln!("Bilinmeyen birim: {}", from); process::exit(1); });
    let (tcat, tbase) = units.get(to).unwrap_or_else(|| { eprintln!("Bilinmeyen birim: {}", to); process::exit(1); });

    if fcat != tcat {
        eprintln!("Hata: '{}' ({}) ve '{}' ({}) farklı kategoriler.", from, fcat, to, tcat);
        process::exit(1);
    }
    let result = val * fbase / tbase;
    println!("{} {} = {:.6} {}", val, from, result, to);
}
