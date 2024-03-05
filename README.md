# DEVOPS - CSGames 2024
<img src="assets/logo.svg" width="300">

Bienvenue à la compétition de devops. Cette compétition a comme but
la mise en pratique de vos connaissances en intégration continue de systèmes modulaires.

Avant de débuter, il est primordiale d'avoir accès aux ressources suivantes :
- Votre numéro d'équipe.
- Votre nom d'utilisateur d'équipe, au format : `team{# d'équipe}@cs2024.one`
- Votre mot de passe d'équipe.

**Ces informations vous serons fournies suite à votre arrivée.**

### Règles générales

- Aucune communication (bidirectionnelle) : il est uniquement permis de communiquer avec les membres de votre équipe.
- Utilisation d'AI générative interdite (ChatGPT, Copilot, ...) : l'utilisation de ces outils sera considérés comme une communication extérieure.
- Ne pas tenter de nuire à l'infrastructure de la compétition.

> Note : Les forums d'aide en ligne (Stack overflow, Reddit, ...) ne sont pas considéré comme de l'aide extérieur. Leur utilisation est donc permise.
> Vous ne pouvez toutefois pas poser de questions sur ces forums.

En cas de non-respect de ces règles des pénalités seront appliqués : **perte de point, disqualification**.

## Introduction et objectif
Certains de vos concitoyens sont resté coincés à l'intérieur d'un bunker dans un lieu isolé et **entouré d'arbres**! Nous appellerons ce lieu "la jungle".
Votre objectif est de les aider en s'en échapper. Pour ce faire, vous devez trouver un moyen de communiquer avec eux.

Le bunker dispose d'un seul accès réseau et celui-ci permet uniquement d'accéder à un service détenu par une IA capricieuse
qui surveille toutes les requêtes. Pour s'échapper et retrouver la civilisation les habitant de la jungle doivent avoir accès aux informations suivantes :

- La météo, les plantes font moins peur quand il fait froid
- Une carte
- Le code d'accès de la porte

Vous avez la chance de disposer d'un accès complet à internet (sauf ChatGPT 🤷) et de manière limitée au cluster contrôlé par l'IA.
Vous pourrez ainsi aider vos concitoyens en leur fournissant les informations dont ils ont besoin!

L'architecture simplifiée peut être résumée par la figure suivante :

![](assets/base-design.svg)

Les habitant de la jungle dispose d'une communication directe à l'IA qui se trouve dans le même cluster.
Votre équipe se trouve dans une autre cluster qui n'est pas limité dans ces requêtes extérieures.
Votre cluster peut communiquer avec l'IA par l'entremise d'un réseau virtuel (VNET) mis en place entre les deux clusters (cette étape est déjà réalisée pour vous).

Bien entendu, le compte Azure fournit donne uniquement accès à votre cluster (Team cluster).
En déployant votre code dans le cluster de votre équipe, vous serez en mesure d'interagir avec la jungle.

Les étapes à réaliser afin d'accomplir votre mission sont listés dans [Épreuves](#épreuves).
Le [guide de départ](#guide-de-départ) décrit la configuration requise pour compléter les épreuves.

Bonne chance!

## Guide de départ

Cette section détails installation des dépendances requises.

Bien que cette compétition puisse être réalisée sur Windows et MacOS, l'utilisation de Linux est encouragée.
Sous Windows, il est possible d'utiliser WSL.

Éditeurs recommandés : VSCode, RustRover (en beta).

Tout d'abord, tentez de vous connecter à Azure avec votre compte d'équipe à l'adresse suivante : https://portal.azure.com.

### Installation des dépendances :
- Docker, engin (de préférence) ou desktop : https://docs.docker.com/engine/install/
- AZ shell, facilite l'accès au cluster : https://learn.microsoft.com/en-us/cli/azure/install-azure-cli
- Helm, permet de tester localement les charts avant de les déployer : https://helm.sh/docs/intro/install/

### Dépendances recommandées :
- Rust : https://www.rust-lang.org/tools/install
- Clippy, permet le formatage du code source : ``rustup component add clippy``

> À noter : L'utilisation de rust n'est pas obligatoire, cependant l'épreuve 4.1 nécessite une puissance de calcul plus élevée.

## Épreuves
Cette section détaille les différentes épreuves de la compétition.

### 1. Code de départ (5 points)
#### 1.1 Serveur HTTP (1.5 point)
Cette première étape est la plus simple. Vous devez mettre en place un serveur http permanent d'obtenir le status de votre service (health check).
L'adresse n'est pas très importante (ex. /health).

#### 1.2 Conteneur Docker (3.5 points)
Vous souhaitez conteneuriser votre implémentation afin de la déployer plus facilement. Une coquille de Dockerfile est fournie.

N'oubliez pas de :
- Permettre l'ouverture des ports nécessaire pour que votre serveur fonctionne
- Séparer votre Dockerfile en plusieurs sections (compilation, exécution, ...)
- Utiliser des images de base sécuritaires et de petite taille

### 2. Déploiement (6 points)
Cette section décris les étapes nécessaires afin de déployer votre conteneur de la section précédente sur votre cluster.

#### 2.1 Helm (2 points)
Vous devez créer des charts helm permettant de déployer votre conteneur sur un cluster Kubernetes.

Les charts doivent permettre de :
- Déployer votre service à partir du ACR (Azure Container Registry) mis à votre disposition (l'image sera y sera poussée en 2.2).
- Ajouter un service kubernetes permettant d'accéder à votre pod.
- Ajouter un Ingress permettant d'accéder à votre service depuis l'extérieur et ainsi intéragir avec ce dernier.
- Ajouter un Service permettant à l'autre cluster (celui de l'IA) d'accéder au vôtre.

L'autre cluster tentera d'accéder au vôtre à adresse : 10.30.10.10.
Le service permettant l'accès à votre pod à partir de l'autre cluster est similaire à celui fournit. Il est toutefois conseillé de jeter un coup d'œil aux annotations suivantes :
```yml
service.beta.kubernetes.io/azure-load-balancer-ipv4: TODO
service.beta.kubernetes.io/azure-load-balancer-internal: TODO
```

Un exemple de chart helm est fourni. Vous pouvez également recréer un template de charts avec la commande ``helm create NAME_OF_PROJECT``.

#### 2.2 Azure (2 point)
Lors de cette étape, vous devez créer une image de votre service, pousser cette image sur l'ACR et déployer votre service à l'aide des charts de l'étape précédente.

Ici, l'objectif n'est pas de déployer à partir de Gitlab, mais bien à partir de votre ordinateur afin de valider que ce que vous avez fait jusqu'à présent est bien fonctionnel.

Les variables nécéssaires au déploiement sont les suivantes :
- Votre nom d'utilisateur Azure
- Votre mot de passe Azure
- Le TenantId (vous devez le retrouver à partir du portail en ligne au de AZ shell)
- Le nom de l'ACR
- Le nom du Ressource Group
- Le nom du cluster
- Le nom du domaine : team{# d'équipe}.dev.cs2024.one

L'ensemble de ces étapes doivent être scripté afin d'être facilement reproductibles. Vous devez vous référer à la documentation de Helm et Azure cli.

> À noter : Un exemple de script (``deploy-aks.sh``) vous est fournis. Vous pouvez partir de là pour cette étape.

#### 2.3 Pipeline Gitlab (2 points)
À partir des étapes de la section 2.2 vous devez automatiser le déploiement par l'entremise d'un pipeline Gitlab.

Le pipeline doit permettre de :
- Compiler le code.
- Vérifier la structure du code (lint). Peut être réalisé avec Clippy si Rust est utilisé.
- Déployer sur le cluster AKS (Azure Kubernetes Cluster).

Les deux premières étapes vous sont laissées. Pour la dernière, voici quelques suggestions :
- Générer l'image docker à partir du pipeline.
- Réutiliser le script de l'étape précédente afin de pousser l'image sur l'ACR et déployer les charts helm.
- Placer les informations nécessaires au déploiement en variable d'environnement dans les configurations de votre dépôt Gitlab.
- Utiliser sur docker-in-docker (dind). Les pipelines Gitlab étant conteneurisé par défaut, dind est très utile afin de permettre l'utilisation de Docker à partir du pipeline.

Exemple pour docker-in-docker :
```yml
JOB_NAME:
  stage: STAGE_NAME
  image: brqu/docker-az:latest # Or docker:24.0.5
  services:
    - docker:24.0.5-dind
  before_script:
    - docker info
...
```

> L'image ``brqu/docker-az:latest`` est basée sur ``docker:24.0.5`` et contient en plus helm et AZ shell. En l'utilisant, vous n'aurez pas besoin d'installer ces outils à chaque déploiement et accélèrerez ainsi votre pipeline.

> Pour vous connecter à Azure à partir du pipeline vous devrez utiliser ``az login`` avec votre nom d'utilisateur et votre mot de passe.

### 3. Accéder à la jungle (2 points)
Dans cette étape, vous devez accéder à la page de status sur service des prisonniers de la jungle. Pour ce faire, vous devez faire une requête http GET à l'adresse suivante à partir de votre service :
- http://ai.private.dev.cs2024.one/jungle

L'information est retournée dans le corps (_body_) de la réponse en JSON. Il s'agit d'une liste de ``Step``. Un ``Step`` est définie selon le format suivant :
```typescript
interface Step {
    name: string;
    status: string;
}
```

Vous devez pouvoir accéder à l'information retournée par cette requête en effectuant une requête à votre propre service.

### 4. Libérer les prisonniers (6 points)
L'objectif de cette section est de fournir les informations nécessaires au service des prisonniers afin qu'ils puissent se libérer.

#### 4.1 Fournir un accès (1 point)
Afin d'accomplir les étapes qui suivent, il est nécessaire que les prisonniers de la jungle soient en mesure de communiquer avec votre équipe.
Pour ce faire, ils effectueront des requêtes à travers l'IA qui seront redirigés vers votre cluster.

Les requêtes seront des ``POST`` vers le chemin suivant : ``/router``.

Chaque requête contient un paramètre (_query parameter_) : ``request``. Ce paramètre permet d'indiquer le type de requête en provenance de la jungle.
Le corps (_body_) de la requête contient de l'information sérialisée au format _JSON_ spécifique à chaque requête.

Afin de valider que vous êtes bien en mesure de recevoir les requêtes de la jungle, il suffit d'écouter à l'address ``/router`` des requêtes ayant comme paramètre ``?request=status``.
Pour indiquer que le message est bien reçu, il suffit de répondre à la requête avec un code d'erreur dans les 200.

#### 4.2 Météo (1.5 point)
Afin de s'échapper de leur bunker, les prisoners doivent avoir accès aux conditions météos. En effet, les plantes aiment beaucoup la chaleur, ils vont donc s'échapper lorsqu'il fait plus froid.

Afin d'obtenir d'obtenir les informations météo, les prisonniers vont réaliser une requête vers le chemin ``/router?request=weather``.

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
> Si vous utilisez une autre API, les résults seront considérés valides si les précipitations et la température sont similaires (5mm, 5°c).

#### 4.3 Carte (2 points)
Afin de sortir de leur bunker, les prisonniers doivent avoir accès à la carte de la jungle. Cette carte prend la forme d'un conteneur docker.

Image de la carte : ``brqu/jungle-map``.

Ce conteneur doit être déployé sur le même cluster que votre conteneur.

> À noter : Il est fortement conseiller d'effectuer le déploiement avec helm (voir ``helm init``).

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

Le conteneur fournit alors une réponse en ``JSON`` suivant le format suivant :

```typescript
SIZE = ...
interface MapResponse {
    map: number[SIZE][SIZE]; // Matrix of intergers
}
```

Cette information peut être directement retournée à la requête (sur ``/router``) car le format de réponse est le même!

Si cette étape est concluante, la page de status (http://ai.private.dev.cs2024.one/jungle) devrait être mise à jour après environ une minute.

#### 4.4 Fournir aux prisoners mot de passe de la porte (1.5 point)
Maintenant que les prisoners ont accès à la météo et à la carte, il ne reste plus qu'à leur donner le mot de passe de la porte qui les séparent du monde extérieur.

Heureusement pour eux, les mots de passes choisis par l'IA sont inspirés de mots de passe réels qui sont présents dans une [liste](https://raw.githubusercontent.com/DavidWittman/wpxmlrpcbrute/master/wordlists/1000-most-common-passwords.txt) bien connue.

Lors qu'il tente d'ouvrir la porte, un mot de passe à usage unique est "généré" à partir de cette liste et les prisonniers ont trouvé le moyen d'intercepter le hash md5 de ce mot de passe!
Cependant, ils ne disposent pas d'une puissance de calcul suffisante afin de retrouver le bon mot de passe en moins de 500ms. Ils ont donc encore besoin de votre aide.

Les mots de passe haché seront transmis encodé au format base64 à ``/router?request=door`` dans un _payload_ au format suivant :

```typescript
interface Door {
    hash: string; // base64 encoded md5 hash
}
```

La jungle n'attend pas de réponse de cette requête, il suffit de répondre avec un code dans les 200.

Vous devez trouver le mot de passe correspond au hash en moins de 500ms et l'envoyer à la jungle.

Afin d'envoyer le mot de passe à la jungle, vous devez effectuer une requête ``POST`` à l'adresse suivante :
```
http://ai.private.dev.cs2024.one/jungle/unlock?password=UNHASHED_PASSWORD
```

Si le mot de passe est correctement retourné, la page de stage de la jungle devrait se mettre à jour en environ une minute.

### 5. Bonus (0.5 points)

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

Les quatre premiers critères sont détaillées dans la section [Épreuves](#épreuves).

Le dernier critère sera évalué en fonction de la cohérence générale de la solution et sera évalué selon les
critères suivants (perte de 1 point au maximum) :
- Aucun linter n'est utilisé : -0.5.
- Aucune convention propre au langage utilisée n'est respectée : -0.5.
- Code dupliqué (ex. ne pas utiliser le script de déploiement dans le pipeline) : -0.5.
- Présence de secrets dans le code (ex. mot de passe) : -0.5.
- Valeurs propres à environment présentes directement dans le code (tentez d'utiliser des variables d'environnement) : -0.25.

> À noter : L'évaluation est partiellement automatisée, néanmoins l'ensemble de votre travail sera révisé manuellement.
> En cas de doute, n'hésitez pas à indiquer vos suppositions dans la section [Commentaires](#Commentaires) à la fin de ce fichier.

### Remise
- La remise se fait par Git, vous devez soumettre par l'entremise de ce dépôt tout votre travail
- Le dernier commit poussé sur la branche `main` sera corrigé
- Tout commit poussé après l'heure de remise sera ignoré

## Commentaires

...
