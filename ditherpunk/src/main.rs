use argh::FromArgs;
use image::io::Reader as ImageReader;
use image::ImageError;
use image::{Rgb, RgbImage, Luma, Pixel};
use rand::Rng;
use image::DynamicImage;


#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette rÃ©duite de couleurs.
struct DitherArgs {

    /// le fichier dâentrÃ©e
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode dâopÃ©ration
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de lâimage par seuillage monochrome.
struct OptsSeuil {
    /// couleur claire en format hexadécimal (par défaut : blanc #FFFFFF)
    #[argh(option, default = "String::from(\"FFFFFF\")")]
    couleur_1: String,

    /// couleur foncée en format hexadécimal (par défaut : noir #000000)
    #[argh(option, default = "String::from(\"000000\")")]
    couleur_2: String,

    /// mode de tramage
    #[argh(option, short = 't')]
    tramage: String, // par exemple "aleatoire", "bayer", "monochrome"
    
    /// ordre de la matrice de Bayer (utilisé si le tramage est "bayer")
    #[argh(option, short = 'o', default = "2")]
    bayer_order: u32, 
}



#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de lâimage avec une palette contenant un nombre limitÃ© de couleurs
struct OptsPalette {

    /// le nombre de couleurs Ã  utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}
 
const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);


fn distance_couleur(c1: Rgb<u8>, c2: Rgb<u8>) -> f64 {
    let r_diff = c1[0] as i32 - c2[0] as i32;
    let g_diff = c1[1] as i32 - c2[1] as i32;
    let b_diff = c1[2] as i32 - c2[2] as i32;
    ((r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as f64).sqrt()
}

fn tramage_aleatoire(img: &mut RgbImage) {
    let mut rng = rand::thread_rng();  // Générateur de nombres aléatoires
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let luminosity = pixel.to_luma()[0]; // Calculer la luminosité du pixel
        let seuil: f64 = rng.gen(); // Générer un seuil entre 0 et 1
        if luminosity as f64 / 255.0 > seuil {  // Comparer la luminosité avec le seuil
            *pixel = WHITE;  // Si la luminosité est supérieure au seuil, rendre le pixel blanc
        } else {
            *pixel = BLACK;  // Sinon, rendre le pixel noir
        }
    }
}

fn generate_bayer_matrix(order: u32) -> Vec<Vec<u8>> {
    if order == 0 {
        return vec![vec![0]];
    }

    let previous_matrix = generate_bayer_matrix(order - 1);
    let size = previous_matrix.len();
    let mut matrix = vec![vec![0; size * 2]; size * 2];

    for i in 0..size {
        for j in 0..size {
            let base_value = previous_matrix[i][j] * 4;
            matrix[i][j] = base_value; // Haut-gauche
            matrix[i][j + size] = base_value + 2; // Haut-droit
            matrix[i + size][j] = base_value + 3; // Bas-gauche
            matrix[i + size][j + size] = base_value + 1; // Bas-droit
        }
    }

    matrix
}


fn tramage_bayer(img: &mut RgbImage, order: u32) {
    let bayer_matrix = generate_bayer_matrix(order);
    let matrix_size = bayer_matrix.len() as u32;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let luminosity = pixel.to_luma()[0] as f64 / 255.0; // Luminosité normalisée (0.0 - 1.0)
        let threshold = bayer_matrix[(y % matrix_size) as usize][(x % matrix_size) as usize] as f64
            / (matrix_size * matrix_size) as f64; // Seuil basé sur la matrice de Bayer
        if luminosity > threshold {
            *pixel = WHITE;
        } else {
            *pixel = BLACK;
        }
    }
}

fn monochrome(img: &mut RgbImage, couleur_1: &str, couleur_2: &str) {
    // Conversion des couleurs hexadécimales en RGB
    let couleur_1 = Rgb([
        u8::from_str_radix(&couleur_1[0..2], 16).unwrap(),
        u8::from_str_radix(&couleur_1[2..4], 16).unwrap(),
        u8::from_str_radix(&couleur_1[4..6], 16).unwrap(),
    ]);
    let couleur_2 = Rgb([
        u8::from_str_radix(&couleur_2[0..2], 16).unwrap(),
        u8::from_str_radix(&couleur_2[2..4], 16).unwrap(),
        u8::from_str_radix(&couleur_2[4..6], 16).unwrap(),
    ]);

    // Parcours de tous les pixels de l'image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Calcul de la luminosité du pixel (composant de gris)
        let luminosity = pixel.to_luma()[0]; // Calculer la luminosité du pixel en utilisant la méthode to_luma
        if luminosity > 127 {
            *pixel = couleur_1; // Si luminosité > 127, on utilise couleur_1
        } else {
            *pixel = couleur_2; // Sinon, on utilise couleur_2
        }
    }
}

fn help() {
    println!("Usage: cargo run -- <input> <mode> [options]");
    println!("Convertit une image en monochrome ou vers une palette réduite de couleurs.");
    println!();
    println!("Arguments:");
    println!("  <input>     le fichier d'entrée");
    println!("  <mode>      le mode d'opération");
    println!();
    println!("Modes d'opération:");
    println!("  seuil       Rendu de l'image par seuillage monochrome");
    println!("  palette     Rendu de l'image avec une palette contenant un nombre limité de couleurs");
    println!();
    println!("Options pour le mode 'seuil':");
    println!("  --tramage <tramage>  mode de tramage (aleatoire, bayer, monochrome)");
    println!("  Options pour le mode 'bayer':");
    println!("    --bayer-order <ordre>  ordre de la matrice de Bayer (par défaut : 2)");
    println!("  Options pour le mode 'monochrome':");
    println!("    --couleur-1 <couleur>  couleur claire en format hexadécimal sans '#' (par défaut : blanc #FFFFFF)");
    println!("    --couleur-2 <couleur>  couleur foncée en format hexadécimal sans '#' (par défaut : noir #000000)");
    println!();
    println!("Options pour le mode 'palette':");
    println!("  --n-couleurs <nombre>  le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]");
}


fn main() -> Result<(), ImageError> {

    // Vérifier si --help est passé comme argument
    if std::env::args().any(|arg| arg == "--help") {
        help();
        return Ok(()); // On termine l'exécution après l'affichage de l'aide
    }

    let args: DitherArgs = argh::from_env();
    let path_in = args.input;

     



    // Partie 1

    // let img_file = ImageReader::open(&path_in)?;
    // let mut img: image::RgbImage = img_file.decode()?.to_rgb8();
    // println!("J'ai chargé une image de largeur {}", img.width());
    // println!("Le pixel en 32, 52 a pour couleur {:?}", img.get_pixel(32, 52));
    // for (x, y, color) in img.enumerate_pixels_mut() {
    //     if (x + y) % 2 == 0 {
    //         *color = Rgb([255, 255, 255])
    //     }
    // }



    let img_file = ImageReader::open(&path_in)?;
    let mut img: image::RgbImage = img_file.decode()?.to_rgb8();
    println!("J'ai chargé une image de largeur {}", img.width());

    match args.mode {
        Mode::Seuil(opts) => {

        
            match opts.tramage.as_str() {
                "aleatoire" => {
                    tramage_aleatoire(&mut img);
                }
                "bayer" => {
                    tramage_bayer(&mut img, opts.bayer_order);
                }
                "monochrome" => {
                    monochrome(&mut img, &opts.couleur_1, &opts.couleur_2);
                }
                _ => {
                    eprintln!("Option de tramage inconnue : {}", opts.tramage);
                }
            }

        }
        Mode::Palette(opts) => {
            // Palette de couleurs disponibles
            let palette = vec![
                BLACK, WHITE, RED, GREEN, BLUE, YELLOW, MAGENTA, CYAN,
            ];

            // Limiter la palette à n_couleurs
            if opts.n_couleurs == 0 {
                eprintln!("Avertissement : la palette est vide. L'image sera renvoyée sans modification.");
                // Aucun traitement nécessaire, l'image reste inchangée
            } else {
                let palette = &palette[..opts.n_couleurs.min(palette.len())];

                // Remplacer chaque pixel par la couleur la plus proche de la palette
                for (x, y, pixel) in img.enumerate_pixels_mut() {
                    let mut min_distance = f64::MAX;
                    let mut nearest_color = BLACK;

                    for &color in palette {
                        let distance = distance_couleur(*pixel, color);
                        if distance < min_distance {
                            min_distance = distance;
                            nearest_color = color;
                        }
                    }

                    *pixel = nearest_color;
                }
            }
        }
    }

    if let Some(output) = args.output {
        println!("J'écris l'image dans le fichier {}", output);
        img.save(output)?;
    } else {
        println!("J'affiche l'image");
        img.save("output/test.png")?;

    }

    Ok(())
}