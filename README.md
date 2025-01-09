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

Si l'image de départ avait un canal alpha, il sera perdu lors de la sauvegarde.

### Question 4


