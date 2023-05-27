use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let pass = generarContrasen();
    println!("{pass}");
}

fn generarContrasen() -> String {
    let pass = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>();
    let caracteres_mayusculas = pass.chars().any(|c| c.is_ascii_uppercase()); //Comprobar si tiene mayusculas.
    loop {
        if caracteres_mayusculas {
            return pass;
        }
    }
}
