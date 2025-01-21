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
