mod lexer;
use lexer::Lexer;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // El primer argumento (args[0]) es siempre la ruta del ejecutable.
    // El segundo argumento (args[1]) debería ser la ruta de nuestro archivo .ni
    if args.len() < 2 {
        eprintln!("Error: No se proporciono ningun archivo.");
        eprintln!("Uso correcto: cargo run -- <ruta_del_archivo.ni>");
        process::exit(1);
    }

    let ruta_archivo = &args[1];

    // Validación opcional
    if !ruta_archivo.ends_with(".ni") {
        eprintln!(
            "Error: El archivo '{}' no tiene la extension permitida (.ni)",
            ruta_archivo
        );
        process::exit(1);
    }

    // Leer el contenido del archivo
    let codigo_nivre = match fs::read_to_string(ruta_archivo) {
        Ok(contenido) => contenido,
        Err(error) => {
            eprintln!("Error al abrir el archivo '{}': {}", ruta_archivo, error);
            process::exit(1);
        }
    };

    println!("Analizando el archivo: {}\n", ruta_archivo);

    // Inicializar el lexer con el contenido real del archivo
    let mut analizador = Lexer::new(&codigo_nivre);
    let tokens = analizador.tokenize();

    // Imprimir la lista de tokens generada
    println!("--- Lista de Tokens ---");
    for token in tokens {
        // Formateamos el enum a texto y lo cortamos en el primer '('
        let nombre_tipo = format!("{:?}", token.token_type);
        let solo_nombre = nombre_tipo.split('(').next().unwrap_or(&nombre_tipo);

        println!(
            "[{}] Valor: '{}' | Línea: {}, Columna: {}, Indentación: {}",
            solo_nombre, token.value, token.line, token.column, token.indent_level
        );
    }
}
