_siez voir ce qui se passe._

L'étape suivante consiste à ajouter un titre et à souligner du texte. Typst utilise un balisage simple pour les tâches de formatage les plus courantes. Pour ajouter un titre, saisissez le `=`caractère et pour souligner un texte en italique, placez-le entre `_underscores_`.

```
= Introduction
In this report, we will explore the
various factors that influence _fluid
dynamics_ in glaciers and how they
contribute to the formation and
behavior of these natural structures.

```

![Aperçu](/assets/hDtb7jRd-u1GxrXbK8IzfAAAAAAAAAAA.png)

C'était facile! Pour ajouter un nouveau paragraphe, ajoutez simplement une ligne vide entre deux lignes de texte. Si ce paragraphe nécessite un sous-titre, produisez-le en tapant `==`à la place de `=`. Le nombre de `=`caractères détermine le niveau d'imbrication du titre.

Nous souhaitons maintenant énumérer quelques-unes des circonstances qui influencent la dynamique des glaciers. Pour ce faire, nous utilisons une liste numérotée. Pour chaque élément de la liste, on tape un `+`caractère en début de ligne. Typst numérotera automatiquement les éléments.

```
+ The climate
+ The topography
+ The geology

```

![Aperçu](/assets/UCD504wZ0vAGWF4mLNbJtgAAAAAAAAAA.png)

Si nous voulions ajouter une liste à puces, nous utiliserions le `-`caractère au lieu du `+`caractère. Nous pouvons également imbriquer des listes : par exemple, nous pouvons ajouter une sous-liste au premier élément de la liste ci-dessus en la mettant en retrait.

```
+ The climate
  - Temperature
  - Precipitation
+ The topography
+ The geology

```

![Aperçu](/assets/MxatP-QgeeFcBXvpqnabgQAAAAAAAAAA.png)

## Ajouter une figure

Vous pensez que votre rapport gagnerait à être chiffré. Ajoutons-en un. Typst prend en charge les images aux formats PNG, JPEG, GIF et SVG. Pour ajouter un fichier image à votre projet, ouvrez d'abord le *panneau de fichiers* en cliquant sur l'icône en forme de boîte dans la barre latérale gauche. Ici, vous pouvez voir une liste de tous les fichiers de votre projet. Actuellement, il n'y en a qu'un : le fichier Typst principal dans lequel vous écrivez. Pour télécharger un autre fichier, cliquez sur le bouton avec la flèche dans le coin supérieur droit. Cela ouvre la boîte de dialogue de téléchargement, dans laquelle vous pouvez sélectionner les fichiers à télécharger depuis votre ordinateur. Sélectionnez un fichier image pour votre rapport.

![Boîte de dialogue de téléchargement](/assets/1-writing-upload.png)

Nous avons déjà vu que des symboles spécifiques (appelés *markup* ) ont une signification spécifique dans Typst. Nous pouvons utiliser `=`, `-`, `+`et `_`pour créer respectivement des titres, des listes et du texte souligné. Cependant, avoir un symbole spécial pour tout ce que nous voulons insérer dans notre document deviendrait bientôt énigmatique et compliqué. Pour cette raison, Typst réserve les symboles de balisage uniquement pour les éléments les plus courants. Tout le reste est inséré avec *des fonctions.* Pour que notre image apparaisse sur la page, nous utilisons [`image`](https://typst-app.translate.goog/docs/reference/visualize/image/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp)la fonction de Typst.

```
#image("glacier.jpg")

```

![Aperçu](/assets/GSAtkgPQ2v4iy41ntRez0QAAAAAAAAAA.png)

En général, une fonction produit une sortie pour un ensemble d' *arguments* . Lorsque vous *appelez* une fonction dans le balisage, vous fournissez les arguments et Typst insère le résultat (la *valeur de retour* de la fonction ) dans le document. Dans notre cas, la `image`fonction prend un argument : le chemin d'accès au fichier image. Pour appeler une fonction en balisage, il faut d'abord taper le `#`caractère, immédiatement suivi du nom de la fonction. Ensuite, nous mettons les arguments entre parenthèses. Typst reconnaît de nombreux types de données différents dans les listes d'arguments. Notre chemin de fichier est une courte [chaîne de texte](https://typst-app.translate.goog/docs/reference/foundations/str/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) , nous devons donc le mettre entre guillemets doubles.

L'image insérée utilise toute la largeur de la page. Pour changer cela, transmettez l' `width`argument à la `image`fonction. Il s'agit d'un argument *nommé* et donc spécifié sous forme de `name: value`paire. S'il y a plusieurs arguments, ils sont séparés par des virgules, nous devons donc d'abord mettre une virgule derrière le chemin.

```
#image("glacier.jpg", width: 70%)

```

![Aperçu](/assets/1wmZ01GbbMLBTgr9nEi8sgAAAAAAAAAA.png)

L' `width`argument est une [longueur relative](https://typst-app.translate.goog/docs/reference/layout/relative/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) . Dans notre cas, nous avons spécifié un pourcentage, déterminant que l'image doit occuper `70%`la largeur de la page. Nous aurions également pu spécifier une valeur absolue comme `1cm`ou `0.7in`.

Tout comme le texte, l'image est désormais alignée par défaut sur le côté gauche de la page. Il manque également une légende. Corrigeons cela en utilisant la fonction [figure](https://typst-app.translate.goog/docs/reference/model/figure/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) . Cette fonction prend le contenu de la figure comme argument de position et une légende facultative comme argument nommé.

Dans la liste d'arguments de la `figure`fonction, Typst est déjà en mode code. Cela signifie que vous pouvez maintenant supprimer le hachage avant l'appel de la fonction image. Le hachage n'est nécessaire que directement dans le balisage (pour lever l'ambiguïté du texte des appels de fonction).

La légende est constituée d'un balisage arbitraire. Pour donner un balisage à une fonction, nous la mettons entre crochets. Cette construction est appelée un *bloc de contenu.*

```
#figure(
  image("glacier.jpg", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
)

```

![Aperçu](/assets/6aMP2cZhNxpMw7GX0knZTQAAAAAAAAAA.png)

Vous continuez à rédiger votre rapport et souhaitez maintenant référencer la figure. Pour ce faire, attachez d'abord une étiquette à la figure. Une étiquette identifie de manière unique un élément de votre document. Ajoutez-en un après la figure en mettant un nom entre crochets. Vous pouvez ensuite référencer la figure dans votre texte en écrivant un `@`symbole suivi de ce nom. Les titres et les équations peuvent également être étiquetés pour les rendre référençables.

```
Glaciers as the one shown in
@glaciers will cease to exist if
we don't take action soon!

#figure(
  image("glacier.jpg", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
) <glaciers>

```

![Aperçu](/assets/_QG3npgVtVe3KLiCE0hqaAAAAAAAAAAA.png)

Jusqu'à présent, nous avons transmis des blocs de contenu (balisage entre crochets) et des chaînes (texte entre guillemets) à nos fonctions. Les deux semblent contenir du texte. Quelle est la différence?

Un bloc de contenu peut contenir du texte, mais aussi tout autre type de balisage, des appels de fonction, etc., alors qu'une chaîne n'est en réalité qu'une *séquence de caractères* et rien d'autre.

Par exemple, la fonction image attend un chemin vers un fichier image. Cela n'aurait pas de sens de passer, par exemple, un paragraphe de texte ou une autre image comme paramètre de chemin d'accès à l'image. C'est pourquoi seules les chaînes sont autorisées ici. Au contraire, les chaînes fonctionnent partout où du contenu est attendu, car le texte est un type de contenu valide.

## Ajouter une bibliographie

Lorsque vous rédigez votre rapport, vous devez sauvegarder certaines de vos affirmations. Vous pouvez ajouter une bibliographie à votre document avec la [`bibliography`](https://typst-app.translate.goog/docs/reference/model/bibliography/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp)fonction. Cette fonction attend un chemin vers un fichier bibliographique.

Le format de bibliographie natif de Typst est [Hayagriva](https://translate.google.com/website?sl=auto&tl=fr&hl=en&client=webapp&u=https://github.com/typst/hayagriva/blob/main/docs/file-format.md) , mais pour des raisons de compatibilité, vous pouvez également utiliser des fichiers BibLaTeX. Comme votre camarade de classe a déjà fait une étude bibliographique et vous a envoyé un `.bib`fichier, vous utiliserez celui-là. Téléchargez le fichier via le panneau de fichiers pour y accéder dans Typst.

Une fois que le document contient une bibliographie, vous pouvez commencer à la citer. Les citations utilisent la même syntaxe que les références à une étiquette. Dès que vous citez une source pour la première fois, elle apparaîtra dans la section bibliographie de votre document. Typst prend en charge différents styles de citation et de bibliographie. Consultez la [référence](https://typst-app.translate.goog/docs/reference/model/bibliography/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp#parameters-style) pour plus de détails.

```
= Methods
We follow the glacier melting models
established in @glacier-melt.

#bibliography("works.bib")

```

![Aperçu](/assets/xklGbWqUimu8pV0wFDkkMQAAAAAAAAAA.png)

## Mathématiques

Après avoir étoffé la section méthodes, vous passez au cœur du document : vos équations. Typst intègre une composition mathématique et utilise sa propre notation mathématique. Commençons par une équation simple. Nous l'enveloppons de `$`signes pour indiquer à Typst qu'il doit s'attendre à une expression mathématique :

```
The equation $Q = rho A v + C$
defines the glacial flow rate.

```

![Aperçu](/assets/xLq1JvBV0YeUBQSmAJsaagAAAAAAAAAA.png)

L'équation est composée en ligne, sur la même ligne que le texte environnant. Si vous souhaitez plutôt l'avoir sur sa propre ligne, vous devez insérer un seul espace au début et à la fin :

```
The flow rate of a glacier is
defined by the following equation:

$ Q = rho A v + C $

```

![Aperçu](/assets/PeZaoVpKqICLGYC48JkubAAAAAAAAAAA.png)

Nous pouvons voir que Typst affichait les lettres simples `Q`, `A`, `v`et `C`telles quelles, tout en les traduisant `rho`en lettre grecque. Le mode mathématique affichera toujours des lettres simples textuellement. Toutefois, plusieurs lettres sont interprétées comme des symboles, des variables ou des noms de fonctions. Pour impliquer une multiplication entre des lettres simples, placez des espaces entre elles.

Si vous souhaitez avoir une variable composée de plusieurs lettres, vous pouvez la mettre entre guillemets :

```
The flow rate of a glacier is given
by the following equation:

$ Q = rho A v + "time offset" $

```

![Aperçu](/assets/pKzq4sozT6I2eUFIrpNDzQAAAAAAAAAA.png)

Vous aurez également besoin d'une formule de somme dans votre article. Nous pouvons utiliser le `sum`symbole puis spécifier la plage de la sommation en indice et en exposant :

```
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla Q_i / 2 $

```

![Aperçu](/assets/oE9DDk_onbKEOhH9DWkFLQAAAAAAAAAA.png)

Pour ajouter un indice à un symbole ou à une variable, saisissez un `_`caractère puis l'indice. De même, utilisez le `^`caractère comme exposant. Si votre indice ou exposant se compose de plusieurs éléments, vous devez les mettre entre parenthèses rondes.

L'exemple ci-dessus nous a également montré comment insérer des fractions : placez simplement un `/`caractère entre le numérateur et le dénominateur et Typst le transformera automatiquement en fraction. Les parenthèses sont intelligemment résolues, vous pouvez donc saisir votre expression comme vous le feriez dans une calculatrice et Typst remplacera les sous-expressions entre parenthèses par la notation appropriée.

```
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla
    (Q_i (a_i - epsilon)) / 2 $

```

![Aperçu](/assets/pMSz49BGbSGdisHfvAechgAAAAAAAAAA.png)

Toutes les constructions mathématiques n'ont pas une syntaxe particulière. Au lieu de cela, nous utilisons des fonctions, tout comme la `image`fonction que nous avons vue précédemment. Par exemple, pour insérer un vecteur colonne, on peut utiliser la [`vec`](https://typst-app.translate.goog/docs/reference/math/vec/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp)fonction. En mode mathématique, les appels de fonction n'ont pas besoin de commencer par le `#`caractère.

```
$ v := vec(x_1, x_2, x_3) $

```

![Aperçu](/assets/vBVlHY0No4dDf6HDafev_AAAAAAAAAAA.png)

Certaines fonctions ne sont disponibles qu'en mode mathématique. Par exemple, la [`cal`](https://typst-app.translate.goog/docs/reference/math/variants/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp#functions-cal)fonction est utilisée pour composer des lettres calligraphiques couramment utilisées pour les ensembles. La [section mathématique de la référence](https://typst-app.translate.goog/docs/reference/math/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) fournit une liste complète de toutes les fonctions rendues disponibles par le mode mathématique.

Encore une chose : de nombreux symboles, comme la flèche, ont de nombreuses variantes. Vous pouvez choisir parmi ces variantes en ajoutant un point et un nom de modificateur au nom d'un symbole :

```typ
$ a arrow.squiggly b $
```

![Aperçu](/assets/fYpgGuOFYIhkxGJrfjHBIQAAAAAAAAAA.png)

Cette notation est également disponible en mode balisage, mais le nom du symbole doit être précédé de `#sym.`là. Consultez la [section Symboles](https://typst-app.translate.goog/docs/reference/symbols/sym/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) pour une liste de tous les symboles disponibles.

## Revoir

Vous avez maintenant vu comment rédiger un document de base dans Typst. Vous avez appris à mettre en valeur du texte, à rédiger des listes, à insérer des images, à aligner du contenu et à composer des expressions mathématiques. Vous avez également découvert les fonctions de Typst. Il existe de nombreux autres types de contenu que Typst vous permet d'insérer dans votre document, tels que [des tableaux](https://typst-app.translate.goog/docs/reference/model/table/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) , [des formes](https://typst-app.translate.goog/docs/reference/visualize/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) et [des blocs de code](https://typst-app.translate.goog/docs/reference/text/raw/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) . Vous pouvez parcourir la [référence](https://typst-app.translate.goog/docs/reference/?_x_tr_sl=auto&_x_tr_tl=fr&_x_tr_hl=en&_x_tr_pto=wapp) pour en savoir plus sur ces fonctionnalités et d'autres.

Pour le moment, vous avez terminé la rédaction de votre rapport. Vous avez déjà enregistré un PDF en cliquant sur le bouton de téléchargement dans le coin supérieur droit. Cependant, vous pensez que le rapport pourrait paraître un peu moins clair. Dans la section suivante, nous apprendrons comment personnaliser l'apparence de notre document.
