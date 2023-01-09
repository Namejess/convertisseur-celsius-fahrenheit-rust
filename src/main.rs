use std::io;

fn main() {
    println!("{}", "Choisissez votre conversion :\n1. Convertir de Celsius a Fahrenheit\n2. Convertir de Fahrenheit a Celsius\n3. Quitter\n");

    let choix: i8 = loop {
        
        let mut menu = String::new();

        io::stdin().read_line(&mut menu).expect("Erreur de lecture");

        let choix: i8 = match menu.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre");
                continue;
            }
        };

        if choix < 1 || choix > 3 {
            println!("Veuillez entrer un nombre entre 1 et 3");
            continue;
        };
        break choix;
    };

    if choix == 1 {
        celsius_fahrenheit();
    } else if choix == 2 {
        fahrenheit_celsius();
    } else {
        println!("Au revoir");
    }

    fn celsius_fahrenheit() {
        let mut temperature = String::new();

        println!("Entrez une temperature en Celsius: ");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Erreur de lecture");

        let temperature: f32 = temperature.trim().parse().unwrap();

        let temperature = temperature * 9.0 / 5.0 + 32.0;

        println!("La temperature en Fahrenheit est: {}", temperature);
    }

    fn fahrenheit_celsius() {
        let mut temperature = String::new();

        println!("Entrez une temperature en Fahrenheit: ");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Erreur de lecture");

        let temperature: f32 = temperature.trim().parse().unwrap();

        let temperature = (temperature - 32.0) * 5.0 / 9.0;

        println!("La temperature en Celsius est: {}", temperature);
    }
}
