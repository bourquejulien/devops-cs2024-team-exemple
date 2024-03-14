# DEVOPS - CSGames 2024
<img src="assets/logo.svg" width="300">

Bienvenue à la compétition de devops. Cette compétition a pour but
la mise en pratique de vos connaissances en intégration continue de systèmes modulaires.

Avant de débuter, il est primordial d'avoir accès aux ressources suivantes :
- Votre numéro d'équipe
- Votre nom d'utilisateur d'équipe au format : ``team{# d'équipe}@cs2024.one``
- Votre mot de passe d'équipe

**Ces informations vous seront fournies au début de la compétition.** Elles vous permettront de vous connecter à ``azure.microsoft.com`` et ``gitlab.com``.

### Règles générales
- Aucune communication (bidirectionnelle) : il est uniquement permis de communiquer avec les membres de votre équipe
- L'utilisation d'IA générative est interdite (ChatGPT, Copilot, ...) : l'utilisation de ces outils sera considérée comme une communication externe
- Ne pas tenter de nuire à l'infrastructure de la compétition

> Note : Les forums d'aide en ligne (Stack overflow, Reddit, ...) ne sont pas considérés comme de l'aide extérieure. Leur utilisation est donc permise.
> Vous ne pouvez toutefois pas poser de questions sur ces forums.

En cas de non-respect de ces règles, des pénalités seront appliquées : **perte de point, disqualification**.

## Introduction et objectif
Certains de vos concitoyens sont coincés à l'intérieur d'un bunker dans un lieu isolé et **entouré d'arbres**! Nous appellerons ce lieu "la jungle".
Votre objectif est de les aider à s'en échapper. Pour ce faire, vous devez trouver un moyen de communiquer avec eux.

Le bunker dispose d'un seul accès réseau et celui-ci permet uniquement d'accéder à un service détenu par une IA capricieuse
qui surveille toutes les requêtes. Pour s'échapper et retrouver la civilisation, les prisonniers du bunker doivent avoir accès aux informations suivantes :

- La météo : les plantes font moins peur quand il fait froid
- Une carte
- Le code d'accès de la porte

Vous avez la chance d'avoir un accès complet à internet (sauf ChatGPT 🤷) et de manière limitée au cluster contrôlé par l'IA.
Vous pourrez ainsi aider vos concitoyens en leur fournissant les informations dont ils ont besoin!

L'architecture simplifiée peut être résumée par la figure suivante :

![](assets/base-design.svg)

Les prisonniers disposent d'une communication directe à l'IA qui se trouve dans le même cluster.
Votre équipe se trouve dans un autre cluster qui n'est pas limité dans ces requêtes extérieures.
Votre cluster peut communiquer avec l'IA par l'entremise d'un réseau virtuel (VNET) mis en place entre les deux clusters (cette étape est déjà réalisée pour vous).

Bien entendu, le compte Azure fourni donne uniquement accès à votre cluster (Team cluster).
En déployant votre code dans le cluster de votre équipe, vous serez en mesure d'interagir avec la jungle.

Les étapes à réaliser afin d'accomplir votre mission sont listées dans [Épreuves](#épreuves).
Le [guide de départ](#guide-de-départ) décrit la configuration requise pour compléter les épreuves.

Bonne chance!

## Guide de départ

Cette section détaille l'installation des dépendances requises.

Bien que cette compétition puisse être réalisée sur Windows et MacOS, l'utilisation de Linux est encouragée.
Sous Windows, il est possible d'utiliser WSL.

Éditeurs recommandés : VSCode, RustRover (en beta).

### Installation des dépendances
- Docker, engin (de préférence) ou desktop : https://docs.docker.com/engine/install/
- AZ cli, facilite l'accès au cluster : https://learn.microsoft.com/en-us/cli/azure/install-azure-cli
- Helm, permet de tester localement les charts avant de les déployer : https://helm.sh/docs/intro/install/
- Rust : https://www.rust-lang.org/tools/install
- Clippy, permets le formatage du code : ``rustup component add clippy``

### Première connexion à Gitlab
Tout d'abord, tentez de vous connecter à Gitlab avec votre compte d'équipe à l'adresse suivante : https://gitlab.com/users/sign_in.

Le nom d'utilisateur est au format : ``team{# d'équipe}@cs2024.one``.

Vous aurez alors accès à un projet au nom de votre équipe : ``team{# d'équipe}``. Tout le code doit être remis dans ce projet.

### Première connexion à Azure
Tentez maintenant de vous connecter à Azure avec votre compte d'équipe à l'adresse suivante : https://portal.azure.com.

Trois ressources vous seront utiles :
- Azure Kubernetes Cluster (AKS) : le cluster sur lequel vous travaillerez se nomme ``teamcluster``
- Azure Container Registry (ACR) : le registre qui contiendra les images de votre service se nomme ``team{TEAM_NUMBER}{RANDOM_ID}``
- Microsoft Entra ID, anciennement active directory : permet de récupérer le Tenant Id

## Épreuves
Cette section détaille les différentes épreuves de la compétition.

À l'exception des scripts de déploiement, tout le code doit être écrit en **Rust**. Les appels à du code écrit en ``C`` / ``C++`` sont interdit.

### 1. Code de départ (5 points)
#### 1.1 Serveur HTTP (1.5 point)
Cette première étape est la plus simple. Vous devez mettre en place un serveur http permettant d'obtenir le statut de votre service (health check).
L'adresse n'est pas très importante (ex. /health).

Vous devrez également vous assurer que le serveur (et le programme) est bien stoppé sur un ``SIGINT`` (Ctrl+C).

#### 1.2 Conteneur Docker (3.5 points)
Vous souhaitez conteneuriser votre implémentation afin de la déployer plus facilement.

N'oubliez pas de :
- Permettre l'ouverture des ports nécessaires pour que votre serveur fonctionne
- Séparer votre Dockerfile en plusieurs sections (compilation, exécution, ...)
- Utiliser le caching afin de réduire le temps de compilation
- Utiliser des images de base sécuritaires et de petite taille (alpine, distroless)
- Lancer le programme à partir d'un utilisateur disposant de peu de privilèges

### 2. Déploiement (6 points)
Cette section décrit les étapes nécessaires afin de déployer votre conteneur de la section précédente sur votre cluster.

#### 2.1 Helm (2 points)
Vous devez créer des charts helm permettant de déployer votre conteneur sur un cluster Kubernetes.

Les charts doivent permettre de :
- Déployer votre service à partir du ACR (Azure Container Registry) mis à votre disposition (l'image y sera poussée en 2.2)
- Ajouter un service kubernetes permettant d'accéder à votre pod
- Ajouter un Ingress permettant d'accéder à votre service depuis l'extérieur et ainsi interagir avec ce dernier
- Ajouter un service permettant à l'autre cluster (celui de l'IA) d'accéder au vôtre

Vous devrez accéder à votre cluster à partir de l'adresse ``http://team{# d'équipe}.dev.cs2024.one/``. Cette adresse doit être indiquée lors du déploiement.

L'autre cluster tentera d'accéder au vôtre à adresse : 10.30.10.10.
Le service permettant l'accès à votre pod à partir de l'autre cluster est similaire à celui fourni. Il est toutefois conseillé de jeter un coup d'œil aux annotations suivantes :
```yml
service.beta.kubernetes.io/azure-load-balancer-ipv4: TODO
service.beta.kubernetes.io/azure-load-balancer-internal: TODO
```

Un exemple de chart helm est fourni. Vous pouvez également recréer un template de charts avec la commande ``helm create NAME_OF_PROJECT``.

#### 2.2 Azure (2 points)
Lors de cette étape, vous devez créer une image de votre service, pousser cette image sur l'ACR et déployer votre service à l'aide des charts de l'étape précédente.

Ici, l'objectif n'est pas de déployer à partir de Gitlab, mais bien à partir de votre ordinateur afin de vérifier si ce que vous avez fait jusqu'à présent est bien fonctionnel.

Les variables nécessaires au déploiement sont les suivantes :
- Votre nom d'utilisateur Azure
- Votre mot de passe Azure
- Le nom de l'ACR
- Le nom du Ressource Group
- Le nom du cluster
- Le nom du domaine, selon vos charts Helm : team{# d'équipe}.dev.cs2024.one

L'ensemble de ces étapes doivent être scriptées afin d'être facilement reproductibles. Vous devez vous référer à la documentation de Helm et Azure cli.

> À noter : Une coquille de script (``deploy-aks.sh``) vous est fournie. Vous pouvez partir de là pour cette étape.

À la fin de cette étape, il devrait être possible d'accéder à votre service (au healthcheck) à partir de ``http://team{# d'équipe}.dev.cs2024.one/{chemin du healthcheck}``.

#### 2.3 Pipeline Gitlab (2 points)
À partir des étapes de la section 2.2 vous devez automatiser le déploiement par l'entremise d'un pipeline Gitlab.

Le pipeline doit permettre de :
- Compiler le code
- Vérifier la structure du code (lint). Peut être réalisé avec Clippy
- Déployer sur le cluster AKS (Azure Kubernetes Cluster)

Les deux premières étapes (build, lint) doivent respecter les requis suivants :
- Doivent être réalisées lors d'un merge request ou lorsque'un commit est poussé sur la branche ``main``
- Utiliser une cache afin d'éviter une recompilation complète lors de chaque exécution

Les requis suivants doivent être respectés pour la dernière étape :
- Doit uniquement être lancé lorsqu'un commit est poussé sur la branche ``main`` ou qu'un tag ``DEPLOY`` est poussé.
- Générer l'image docker à partir du pipeline
- Réutiliser le script de l'étape précédente afin de pousser l'image sur l'ACR et de déployer les charts helm
- Placer les informations nécessaires au déploiement en variable d'environnement dans les configurations de votre dépôt Gitlab
- Utiliser docker-in-docker (dind). Les pipelines Gitlab étant conteneurisés par défaut, dind est très utile afin de permettre l'utilisation de Docker à partir du pipeline

Exemple pour docker-in-docker :
```yml
JOB_NAME:
  stage: STAGE_NAME
  image: brqu/docker-az:latest # Or docker:24.0.5
  services:
    - docker:24.0.5-dind
#  ...
```

> L'image ``brqu/docker-az:latest`` est basée sur ``docker:24.0.5`` et contient Helm et AZ client. En l'utilisant, vous n'aurez pas besoin d'installer ces outils à chaque déploiement et accélèrerez ainsi votre pipeline.

### 3. Accéder à la jungle (2 points)
Dans cette étape, vous devez accéder à la page de statut sur le service des prisonniers. Pour ce faire, vous devez faire une requête http GET à l'adresse suivante à partir de votre service :
- http://ai.private.dev.cs2024.one/jungle

L'information est retournée dans le corps (_body_) de la réponse en JSON. Il s'agit d'une liste de ``Step``. Un ``Step`` est défini selon le format suivant :
```typescript
interface Step {
    name: string;
    status: string;
}
```

Vous devez pouvoir accéder à l'information retournée par cette requête en effectuant une requête à votre propre service. L'information doit être formatée dans une interface conviviale. Cette interface doit comprendre :
- Un tableau html contenant les résultats
- Un compteur indiquant le nombre de ``status`` contenant "✅ OK".
- Le logo contenu dans ``assets/logo.svg``, retourné suite à une requête à votre serveur
- Un bouton permettant d'actualiser la page

### 4. Libérer les prisonniers (6 points)
L'objectif de cette section est de fournir les informations nécessaires au service des prisonniers afin qu'ils puissent se libérer.

#### 4.1 Fournir un accès (1 point)
Afin d'accomplir les étapes qui suivent, il est nécessaire que les prisonniers du bunker soient en mesure de communiquer avec votre équipe.
Pour ce faire, ils effectueront des requêtes à travers l'IA qui seront redirigées vers votre cluster.

Les requêtes seront des ``POST`` vers le chemin suivant : ``/router``.

Chaque requête contient un paramètre (_query parameter_) : ``request``. Ce paramètre permet d'indiquer le type de requête en provenance de la jungle.
Le corps (_body_) de la requête contient de l'information sérialisée au format _JSON_ spécifique à chaque requête.

Afin de valider que vous êtes bien en mesure de recevoir les requêtes de la jungle, il suffit d'écouter à l'adresse ``/router`` les requêtes ayant comme paramètre ``?request=status``.
Pour indiquer que le message est bien reçu, il suffit de répondre à la requête avec un code d'erreur dans les 200.

#### 4.2 Météo (1 point)
Afin de s'échapper de leur bunker, les prisonniers doivent avoir accès aux conditions météo. En effet, les plantes aiment beaucoup la chaleur, ils vont donc s'échapper lorsqu'il fait plus froid.

Pour obtenir les informations météo, les prisonniers vont réaliser une requête vers le chemin ``/router?request=weather``.

Le corps de la requête contiendra les coordonnées du lieu dont ils souhaitent obtenir la météo. Le _payload_ est au format _JSON_ suivant :
```typescript
interface Coords {
    x: number; // latitude
    y: number; // longitude
}
```

Vous devez retourner les informations météo (en réponse à la requête) au format JSON suivant :
```typescript
export interface Weather {
    temperature: number; // Celcius
    windSpeed: number; // Km/h
    precipitation: number; // mm
}
```

> Afin d'obtenir les informations météo, il est suggéré d'utiliser l'API suivante : https://api.open-meteo.com.
> Si vous utilisez une autre API, les résultats seront valides si les précipitations et la température sont similaires (5mm, 5°c).

#### 4.3 Carte (1 point)
Afin de sortir de leur bunker, les prisonniers doivent avoir accès à la carte de la jungle. Cette carte prend la forme d'un conteneur docker.

Image de la carte : ``brqu/jungle-map``.

Ce conteneur doit être déployé sur le même cluster que votre conteneur.

> À noter : Il est fortement conseillé d'effectuer le déploiement avec helm (voir ``helm init``).

Les requêtes en provenance de la jungle (vers ``/router``) auront comme paramètre ``request=map``.
Le payload du _body_ sera au format suivant :
```typescript
interface MapRequest {
    x: number, // latitude, float
    y: number, // longitude, float
    size: number, // map size, positive integer
}
```

Afin d'obtenir une carte à partir de ce conteneur, il est possible d'effectuer une requête ``POST`` à ``/``.
Les paramètres sont des _query parameters_ portant les mêmes noms que les attributs de l'interface ``MapRequest``. Par exemple :

```
http://[MAP_CONTAINER_URL]/?x=75.653&y=-45.6534&size=3
```

Le conteneur fournit alors une réponse en ``JSON`` selon le format suivant :

```typescript
SIZE = ...
interface MapResponse {
    map: number[SIZE][SIZE]; // Matrix of intergers
}
```

Cette information peut être directement retournée à la requête (sur ``/router``) car le format de réponse est le même!

Si cette étape est concluante, le statut (http://ai.private.dev.cs2024.one/jungle) devrait être mise à jour après environ une minute.

#### 4.4 Ouverture de la porte (1 point)
Maintenant que les prisonniers ont accès à la météo et à la carte, il ne reste plus qu'à leur donner le mot de passe de la porte qui les sépare du monde extérieur.

Heureusement pour eux, les mots de passe choisis par l'IA sont inspirés de mots de passe réels qui sont présents dans une liste bien connue (voir le fichier ``passwords.txt``).

Lorsqu’ils tentent d'ouvrir la porte, un mot de passe à usage unique est "généré" à partir de cette liste. Les prisonniers ont trouvé le moyen d'intercepter le hash md5 de ce mot de passe!
Cependant, ils ne disposent pas d'une puissance de calcul suffisante afin de retrouver le bon mot de passe en moins de 500ms. Ils ont donc encore besoin de votre aide.

Les mots de passe hachés seront transmis encodés au format base64 à ``/router?request=door`` dans un _payload_ au format suivant :

```typescript
interface Door {
    hash: string; // base64 encoded md5 hash
}
```

La jungle n'attend pas de réponse de cette requête, il suffit de répondre avec un code dans les 200.

Vous devez trouver le mot de passe qui correspond au hash en moins de 500ms et l'envoyer à la jungle.

Afin d'envoyer le mot de passe à la jungle, vous devez effectuer une requête ``POST`` à l'adresse suivante :
```
http://ai.private.dev.cs2024.one/jungle/unlock?password=UNHASHED_PASSWORD
```

Si le mot de passe est correctement retourné, la page de status de la jungle devrait se mettre à jour environ en une minute.

Finalement, pour retracer les mots de passes utilisés par l'IA, vous devez lister les 10 derniers mots de passes ainsi que le hash qui leur est associé.
La liste doit être triée du mot de passe le plus récent au mot de passe le plus ancien. Cette liste doit être accessible à partir du chemin ``/decrypted-passwords``.

#### 4.5 Gain de popularité (2 points)

> Cette étape doit uniquement être effectuée une fois toutes les étapes précédentes réussies.
> Afin de clairement identifier le déploiement réalisé dans cette section, vous devez placer les charts dans un dossier nommé ``popularity``. Ne modifiez pas directement les charts de la section 2.1.

Votre opération de secours a gagné en popularité auprès des humains résistants aux plantes. Ils suivent les avancées de l'opération!
Cette popularité est telle que les accès à votre service ont grandement augmenté. Vous devez trouver un moyen d'éviter que le ralentissement de votre service entraine l'échec de votre mission.
Cependant, la popularité de la mission assure son financement, le service doit donc rester accessible.

**A) - Load balancer**

La première étape permettant d'assurer la stabilité du service consiste tout simplement à le déployer plusieurs fois.

Vous devez mettre en place un *loadbalancer* kubernetes permettant d'accéder à trois pods plutôt qu'un seul.
Les trois pods doivent continuer à être accessibles à partir des mêmes chemins, comme si un seul pod était déployé.

Afin de limiter les accès à l'IA, vous devez également implémenter une cache dans votre service. Cette dernière doit permettre de stocker la dernière requête effectuée à la page de status de l'IA (http://ai.private.dev.cs2024.one/jungle).
Pour mettre à jour les données en cache, votre service doit réaliser une requête à la page de status (http://ai.private.dev.cs2024.one/jungle) à toutes les 15 secondes. Les accès à ``/jungle-status`` doivent retourner les status en cache.
Si les status ne sont pas encore en cache, la réponse doit attendre. La requête doit aboutir lorsque les résultats sont disponibles.

**B) - Rate limit**

Vous souhaitez limiter les accès en provenance d'un certain bloc d'ips d'où proviennent la majorité des requêtes : ``132.207.0.0/16``.
Pour ce faire, vous devez modifier la configuration de l'_ingress_ Kubernetes afin de limiter à 10 requêtes par seconde le bloc d'Ips ``132.207.0.0/16``.

Vous devez respecter les contraintes suivantes :
- La limite doit uniquement s'appliquer au bloc d'ip mentionné ci-dessus
- Un code d'erreur 503 doit être retourné si la requête est limité
- Le ``rate limit`` doit être appliqué par l'ingress et non par le pod

Vous n'êtes pas obligé d'utiliser un ingress nginx, bien que cela soit conseillé.

### 5. Bonus (0.5 point)

Où se trouve le bunker ?

> ?

## Évaluation et remise

Les critères d'évaluation sont les suivants :

| Critères                | Score /20 |
|-------------------------|-----------|
| Code de base et Docker  | /5        |
| Déploiement             | /6        |
| Accéder à la jungle     | /2        |
| Libérer les prisonniers | /6        |
| Qualité de la solution  | /1        |

Les quatre premiers critères sont détaillés dans la section [Épreuves](#épreuves).

Le dernier critère sera évalué en fonction de la cohérence générale de la solution et sera évalué selon les
critères suivants (perte d'un point au maximum) :
- Aucun linter n'est utilisé : -0.5
- Code dupliqué (ex. ne pas utiliser le script de déploiement dans le pipeline) : -0.5
- Présence de secrets dans le code (ex. mot de passe) : -0.5
- Valeurs propres à environnement présentes directement dans le code (tentez d'utiliser des variables d'environnement) : -0.25

> À noter : L'évaluation est partiellement automatisée, néanmoins l'ensemble de votre travail sera révisé manuellement.

### Remise
- La remise se fait par Git, vous devez soumettre par l'entremise de ce dépôt tout votre travail
- Le dernier commit poussé sur la branche `main` sera corrigé
- Tout commit poussé après l'heure de remise sera ignoré
