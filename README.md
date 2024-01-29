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

Vous avez la chance de disposer d'un accès complet à internet (sauf ChatGPT 🤷) et au cluster contrôlé par l'IA.
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

> À noter : L'utilisation de rust n'est pas obligatoire, cependant certains problèmes nécessite une puissance de calcul élevée.

## Épreuves
Cette section détaille les différentes épreuves de la compétition.

### 1.1 Serveur HTTP
Cette première étape est la plus simple. Vous devez mettre en place un serveur http permanent d'obtenir le status de votre service (health check).
L'adresse n'est pas très importante (ex. /health).

### 1.2 Conteneur Docker
Vous souhaitez conteneuriser votre implémentation afin de la déployer plus facilement. Une coquille de Dockerfile est fournie.

N'oubliez pas de :
- Permettre l'ouverture des ports nécessaire pour que votre serveur fonctionne
- Séparer votre Dockerfile en plusieurs sections (compilation, exécution, ...)
- Utiliser des images de base sécuritaires et de petite taille

### 2. Déploiement
Cette section décris les étapes nécéssaires afin de déployer votre conteneur de la section précédente sur votre cluster.

#### 2.1 Helm
Vous devez créer des charts helm permettant de déployer votre conteneur sur un cluster Kubernetes.

Les charts doivent permettre de :
- Déployer votre service à partir du ACR (Azure Container Registry) mis à votre disposition.
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

#### 2.2 Azure
Lors de cette étape, vous devez créer une image de votre service, pousser cette image sur l'ACR et déployer votre service à l'aide des charts de l'étape précédente.

Ici, l'objectif n'est pas de déployer à partir de Gitlab, mais bien à partir de votre ordinateur afin de valider que ce que vous avez fait jusqu'à présent est bien fonctionnel.

TODO : Décider si le script est fourni...

Les variables nécéssaires au déploiement sont les suivantes :
- Votre nom d'utilisateur Azure
- Votre mot de passe Azure.
- Le TenantId (vous devez le retrouver à partir du portail en ligne au de AZ shell).
- Le nom de l'ACR.
- Le nom du Ressource Group.
- Le nom du cluster.
- Le nom du domaine : team{# d'équipe}.dev.cs2024.one.

#### 2.3 Pipeline Gitlab
À partir des étapes de la section 2.2 vous devez automatiser le déploiement par l'entremise d'un pipeline Gitlab.

Le pipeline doit permettre de :
- Compiler le code.
- Vérifier la structure du code (lint). Peut être réalisé avec Clippy si Rust est utilisé.
- Déployer sur le cluster AKS (Azure Kubernetes Cluster).

Les deux premières étapes vous sont laissées. Pour la dernière, voici quelques suggestions :
- Générer l'image docker à partir du pipeline.
- Réutiliser le script de l'étape précédente afin de pousser l'image sur l'ACR et déployer les charts helm.
- Placer les informations nécéssaires au déploiement en variable d'environnement dans les configurations de votre dépôt Gitlab.
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

### 3. Accéder à la jungle
Dans cette étape, vous devez accéder à la page de status sur service des prisonniers de la jungle. Pour ce faire, vous devez faire une requête http GET à l'adresse suivante à partir de votre service :
- http://ai.private.dev.cs2024.one/jungle

Vous devez pouvoir accéder à l'information retournée par cette requête en effectuant une requête à votre propre service.

### 4. Libérer les prisonniers
L'objectif de cette section est de fournir les informations nécéssaires au service des prisonniers afin qu'ils puissent se libérer.

#### 4.1 Fournir un accès
TODO Readme

#### 4.2 Météo
TODO Readme

#### 4.3 Carte
TODO implémentation

#### 4.4 Code
TODO implémentation

### 5. Bonus

Où se trouve le bunker ?

> ?

## Évaluation et remise

Les critères d'évaluation sont les suivants :

| Critères                | Score /20 |
|-------------------------|-----------|
| Code de base et Docker  | /5        |
| Déploiement             | /5        |
| Accéder à la jungle     | /2        |
| Libérer les prisonniers | /6        |
| Qualité de la solution  | /2        |

Les quatre premiers critères sont détaillées dans la section [Épreuves](#épreuves).
Le dernier critère est beaucoup plus subjectif et sera évalué en fonction de la cohérence générale de la solution.
Il ne s'agit pas proprement dit d'évaluer la qualité du code, mais plutôt du fonctionnement général. Des points seront retranché
en cas de non-respect (évident) de bonne pratique ou encore l'utilisation de hack pouvant être évités.

> À noter : L'évaluation est partiellement automatisée, néanmoins l'ensemble de votre travail sera révisé manuellement.
> En cas de doute, n'hésitez pas à indiquer vos suppositions en commentaire.

### Remise
- La remise se fait par Git, vous devez soumettre par l'entremise de ce dépôt tout votre travail
- Le dernier commit poussé sur la branche `main` sera corrigé
- Tout commit poussé après l'heure de remise sera ignoré

## Erreurs fréquentes
TODO ?
