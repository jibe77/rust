Ce repo contient le code source utilisé lors de ma formation sur le langage Rust.

On retrouve de nombreuses fonctionnalités présentes dans des langages comme Java.
Cependant la particularité de ce langage.
Il utilise des variables immutables pour des raisons de Fearless Concurrency.
Il permet également de manipuler des références sur les variables.
Le mécanisme de frozing, c'est à dire que tant qu'une référence mutable (&mut T) existe, la valeur originale ne peut pas être utilisée directement. Cette règle s'appelle le borrow checking. Cela pourrait créer des "data races" (conditions de concurrence) et des incohérences, où y et x tenteraient d'accéder ou 
de modifier la même donnée de manière non synchronisée.
Ce langage contient des avancées modernes comme le pattern matching.
Il offre la possibilité d'utiliser des closures, ce qui implique la possibilité de la programmation fonctionnelle avec des expresssions lambda.

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

Le répertoire 17_struct indique comment créer un modèle objet en utilisant des structs.

Le répertoire 18_unsafe_static décrit l'utilisation du mot clé static afin de déclarer des variables globales existant pendant toute la vie du programme.

Le répertoire 19_scope explique le mécanisme de frozing, c'est à dire que tant qu'une référence mutable (&mut T) existe, la valeur originale ne peut pas être utilisée directement. Cette règle s'appelle le borrow checking.

Le répertoire 20_cast permet de montrer comment jongler entre les types de données.

Le répertoire 21_if_else montre comment utiliser les boucles if et else.

Le répertoire 22_loops montre comment utiliser des boucles avec le mot clé loop et while.

Le répertoire 24_match montre comment utiliser le pattern matching.

Le répertoire 25_enums montre l'utlisation des énumérations.

Le répertoire 27_functions montre la création de fonctions.

Le répertoire 28_associated_fn_and_meth explique la créations de fonctions dans le cadre de structures.

Le répertoire 29_crates_and_cargo explique l'utilisation de cargo afin d'automatiser la gestion de projets, ainsi que des crates afin d'importer des librairies externes.

Le répertoire 30_closures explique l'utilisation de closures, c'est une notation particulière qui permet de faire notamment de la programmation fonctionnelle.

Le répertoire 31_modules explique le fonctionnement des modules, afin d'organiser son code.

Le répertoire 33_cfg décrit l'annotation de compilation #[cfg(...)] afin de donner des indications de compilation, notamment pour donner des directives en fonction du système cible.

Le répertoire 34_lambda_iterator montre l'utilisation de lambda afin de faire des itérations sur des vecteurs.

Le répertoire 35_errors décrit de diverses façon la moyen de gérer les erreurs :
  * main.rs montre le fonctionnement d'un crash d'une application, en libérant les ressources de la stack d'appels,
  * option.rs montre l'utilisation de Some afin de wrapper un contenu, et ainsi gérer les cas où la valeur n'est pas spécifiée,
  * result.rs montre l'utilisation de Result qui est une énumération générique qui a deux variantes :
    Ok(T) : Indique que l'opération a réussi. La valeur T est le résultat attendu.
    Err(E) : Indique que l'opération a échoué. La valeur E est l'erreur qui a eu lieu.

Le répertoire 36_generics montre l'utilisation des generics sur les fonctions et les structs afin de s'adapter à différents types de paramètres.

Le répertoire 37_ownership décrit le fonctionnement de l'ownership sur les variables, afin d'éviter le mécanisme de garbage collector.
Il précise également le mode de fonctionnement de la stack et de la heap au niveau de la gestion de la mémoire.
On y étudie également l'utilisation de l'apostrophe pour indique au compilateur le cycle de vie des variables.

Le répertoire 38_traits décrit l'utilisation des traits, comme des interfaces en Java, contient des caractéristiques partagées que les types peuvent implémenter.

Le répertoire 39_boxes décrit l'utilisation des boxes. Un "box" (Box<T>) est un pointeur intelligent qui vous permet de stocker des données sur le tas (heap) plutôt que sur la pile (stack). C'est utile lorsque vous manipulez de grandes structures pour éviter les copies coûteuses.
