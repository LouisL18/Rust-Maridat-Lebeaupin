use argh::FromArgs;
use image::io::Reader as ImageReader;
use image::ImageError;
use image::{Rgb, RgbImage, Luma, Pixel};


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

fn main() -> Result<(), ImageError> {
    
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


    // Partie 2 passage en monochrome

    let img_file = ImageReader::open(&path_in)?;
    let mut img: image::RgbImage = img_file.decode()?.to_rgb8();
    println!("J'ai chargé une image de largeur {}", img.width());

    match args.mode {
        Mode::Seuil(opts) => {
            // Passage en monochrome
            let couleur_1 = Rgb([
                u8::from_str_radix(&opts.couleur_1[0..2], 16).unwrap(),
                u8::from_str_radix(&opts.couleur_1[2..4], 16).unwrap(),
                u8::from_str_radix(&opts.couleur_1[4..6], 16).unwrap(),
            ]);
            let couleur_2 = Rgb([
                u8::from_str_radix(&opts.couleur_2[0..2], 16).unwrap(),
                u8::from_str_radix(&opts.couleur_2[2..4], 16).unwrap(),
                u8::from_str_radix(&opts.couleur_2[4..6], 16).unwrap(),
            ]);
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let luminosity = pixel.to_luma()[0]; // Calculer la luminosité
                if luminosity > 127 {
                    *pixel = couleur_1; // Plus de 50 % de luminosité -> couleur claire
                } else {
                    *pixel = couleur_2; // Moins de 50 % de luminosité -> couleur foncée
                }
            }
        }
        Mode::Palette(opts) => {
            // Ici, ajouter le traitement pour la palette (si nécessaire)
            unimplemented!();
        }
    }

    if let Some(output) = args.output {
        println!("J'écris l'image dans le fichier {}", output);
        img.save(output)?;
    } else {
        println!("J'affiche l'image");
        img.save("output/exercice2.8.png")?;

    }

    Ok(())
}