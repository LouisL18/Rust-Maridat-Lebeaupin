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
struct OptsSeuil {}


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

    let img_file = ImageReader::open(&path_in)?;
    let mut img: image::RgbImage = img_file.decode()?.to_rgb8();
    println!("J'ai chargé une image de largeur {}", img.width());
    println!("Le pixel en 32, 52 a pour couleur {:?}", img.get_pixel(32, 52));
    for (x, y, color) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *color = Rgb([255, 255, 255])
        }
    }

    if let Some(output) = args.output {
        println!("J'écris l'image dans le fichier {}", output);
        img.save(output)?;
    } else {
        println!("J'affiche l'image");
        img.save("output/exercice1.5.png")?;

    }

    Ok(())
}