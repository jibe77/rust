Ce repo contient le code source utilisé lors de ma formation sur le langage Rust.

La particularité de ce langage est qu'il utilise des variables immutables pour des raisons de Fearless Concurrency.
Il permet également de manipuler des références sur les variables.

Le répertoire 06_first_rust indique la façon dont on utilise la fonction println! et comment lancer le programme via la commande $ rustc hello.rs && ./hello

Le répertoire 09_variables contient du code indiquant les types de variables.
La décision de rendre les variables immutables par défaut est un choix de conception 
fondamental en Rust, principalement motivé par la sécurité et la concurrence
nommé Fearless Concurrency.

Le répertoire 12_literals présente certains aspects des variables, comme le nommage avec le préfixe _ ou les opérations sur les binaires.

Le répertoire 14_tuples présente l'utilisation des tuples. 
Un tuple en Rust est un moyen de regrouper un nombre fixe de valeurs de différents types dans une seule entité.

Le répertoire 15_arrays indique la façon de manipuler les tableaux.
Un tableau est une collection de taille fixe où tous les éléments doivent avoir le même type.

Le répertoire 16_string_ref indique la façon de manipuler les chaines de caractère, notamment en utilisant des références.

Le répertoire 17_struct indique comment créer un modèle objet.
