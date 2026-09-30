#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)] // À retirer si documentation publique stricte

//! Socle d'application Rust robuste et optimisé (Zero-cost abstractions).
//! Ce module privilégie la gestion explicite des erreurs pour éviter les crashs silencieux.

use std::env;
use std::process;

/// Type de retour standardisé pour centraliser la propagation des erreurs.
type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Isole la logique métier pour faciliter les tests unitaires et la propagation des erreurs (opérateur ?).
fn run(mut args: impl Iterator<Item = String>) -> AppResult<()> {
    // On ignore le premier argument (le nom de l'exécutable)
    args.next();

    let param = match args.next() {
        Some(p) => p,
        None => return Err("Paramètre manquant. Usage : <binaire> <paramètre>".into()),
    };

    println!("Exécution mémoire-safe avec le paramètre : {}", param);
    
    // Itération rapide (Ship-measure-tweak) : ta logique métier s'insère ici.
    
    Ok(())
}

fn main() {
    // Le main agit uniquement comme un routeur d'exécution et de codes de retour système.
    if let Err(e) = run(env::args()) {
        eprintln!("Erreur fatale : {}", e);
        process::exit(1);
    }
}
