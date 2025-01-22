# Rust-Maridat-Lebeaupin

## Partie 1 - La bibliothèque image

### Question 1


Commande pour créer un projet Cargo :

```sh
cargo init ditherpunk
```

Pour ajouter une dépendance sur la bibliothèque image :

```sh
cargo add image
```

Modifier la version dans le fichier `Cargo.toml` :
```toml
[dependencies]
image = "0.24"
```


### Question 2

Pour ouvrir une image depuis un fichier, on utilise la fonction `open`. Pour obtenir une image en mode `RGB8`, on peut utiliser la méthode `to_rgb8` de l'objet `DynamicImage`.

	
### Question 3

Si l'image de départ avait un canal alpha, celui-ci sera perdu lors de la sauvegarde.

### Question 4

Pour savoir trouver la couleur d'un pixel, voici la commande utilisé :
```rs
println!("Le pixel en 32, 52 a pour couleur {:?}", img.get_pixel(32, 52));
```

Résultat :
```
Le pixel en 32, 52 a pour couleur Rgb([8, 8, 8])
```

### Question 5

Pour passer un pixel sur deux d’une image en blanc. Il faut parcourir tous les pixels de l'image :
```rs
for (x, y, color) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *color = Rgb([255, 255, 255]) // pour mettre la couleur du pixel en blanc
        }
    }
```

L'image reste tout de même reconnaissable, on dirait qu'un filtre blanc est appliqué.

## Partie 2 - Passage en monochrome par seuillage

### Question 6

La commande pour récupérer la luminosité d'un pixel utilise la méthode to_luma fournie par le trait Pixel de la bibliothèque image :
```rs
let luminosity = pixel.to_luma()[0];
```

Source :

```
https://docs.rs/image/latest/image/trait.Pixel.html
```

### Question 7

Pour convertir l’image en monochrome, nous avons :

- Parcouru chaque pixel de l’image.

- Calculé la luminosité du pixel avec to_luma.

- Comparé la luminosité à un seuil de 50 % (127 sur une échelle de 0 à 255) :

  - Si la luminosité est supérieure à 127, le pixel est remplacé par la couleur claire.

  - Sinon, il est remplacé par la couleur foncée.

Exemple de code :
```rs
for (x, y, pixel) in img.enumerate_pixels_mut() {
    let luminosity = pixel.to_luma()[0];
    if luminosity > 127 {
        *pixel = couleur_1;
    } else {
        *pixel = couleur_2;
    }
}
```



### Question 8

Pour permettre à l’utilisateur de choisir deux couleurs, nous avons modifié la structure OptsSeuil :
```rs
struct OptsSeuil {
    /// couleur claire en format hexadécimal (par défaut : blanc #FFFFFF)
    #[argh(option, default = "String::from(\"FFFFFF\")")]
    couleur_1: String,

    /// couleur foncée en format hexadécimal (par défaut : noir #000000)
    #[argh(option, default = "String::from(\"000000\")")]
    couleur_2: String,
}
```

Ce code permet de :

- Spécifier les couleurs claires et foncées en ligne de commande sous forme hexadécimale.

- Convertir ces valeurs en Rgb<u8> dans la logique du programme :

```rs
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
```


