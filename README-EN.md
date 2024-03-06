# DEVOPS - CSGames 2024
<img src="assets/logo.svg" width="300">

Welcome to the DevOps competition. This competition aims to put your knowledge of continuous integration of modular systems into practice.

Before starting, it is essential to have access to the following resources:
- Your team number.
- Your team username, in the format: ``team{team number}@cs2024.one``
- Your team password.

**This information will be provided to you at the start of the competition.**

### General rules

- No bidirectional communication: you are only allowed to communicate with your team members.
- The use of generative AI is prohibited (ChatGPT, Copilot, ...): the use of these tools will be considered as external communication.
- Do not attempt to harm the competition infrastructure.

> Note: Online help forums (Stack Overflow, Reddit, ...) are not considered external help. Their use is therefore allowed.
> However, you cannot ask questions on these forums. In case of non-compliance with these rules, penalties will be applied: point deduction, disqualification.

In case of non-compliance with these rules, penalties will be applied: **loss of points, disqualification**.

## Introduction and Objective
Some of your fellow citizens are trapped inside a bunker in an isolated location surrounded by trees! Your objective is to help them escape.
To do this, you must find a way to communicate with them. The bunker has only one network access, which only allows access to a service owned by a capricious AI that monitors all requests.
To escape and find civilization, the inhabitants of the jungle must have access to the following information:
- The weather, plants are less scary when it's cold
- A map
- The door access code

You are lucky, and have full access to the internet (except ChatGPT 🤷), you also have incomplete access to the cluster controlled by the AI.
You will be able to help your fellow citizens by providing them with the information they need!

Simplified architecture can be summarized by the following figure:

![](assets/base-design.svg)

The inhabitants of the jungle have direct communication with the AI located in the same cluster.
Your team is in another cluster that is not limited in its external requests.
Your cluster can communicate with the AI through a virtual network (VNET) set up between the two clusters (this step is already done for you).

Of course, the Azure account provided only gives access to your cluster (Team cluster). By deploying your code to your team's cluster, you will be able to interact with the jungle.

The steps to accomplish your mission are listed in [Challenges](#challenges). The [getting started guide](#getting-started-guide) describes the configuration required to complete the challenges.

Good luck!

## Starting guide

This sections details the installation of Required Dependencies

Although this competition can be done on Windows and MacOS, the use of Linux is encouraged. On Windows, you can use WSL.

Recommended editors: VSCode, RustRover.

First, try to connect to Azure with your team account at the following address: [https://portal.azure.com](https://portal.azure.com).

### Dependency Installation
- Docker, engine (preferably) or desktop: [https://docs.docker.com/engine/install/](https://docs.docker.com/engine/install/)
- AZ cli, facilitates access to the cluster: [https://learn.microsoft.com/en-us/cli/azure/install-azure-cli](https://learn.microsoft.com/en-us/cli/azure/install-azure-cli)
- Helm, allows testing charts locally before deploying them: [https://helm.sh/docs/intro/install/](https://helm.sh/docs/intro/install/)
- Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Clippy, enables source code formatting: `rustup component add clippy`

### First connection to Azure
First, try to connect to Azure with your team account at the following address: [https://portal.azure.com](https://portal.azure.com).

Three resources will be useful to you:
- Azure Kubernetes Cluster (AKS): the cluster you will work on is named ``teamcluster``.
- Azure Container Registry (ACR): the registry that will contain the images of your service is named ``team{TEAM_NUMBER}{RANDOM_ID}``.
- Microsoft Azure AD (formerly Active Directory): allows you to retrieve the Tenant Id.

## Challenges
This section details the different challenges of the competition.

### 1. Starting Code (5 points)
#### 1.1 HTTP Server (1.5 points)
This first step is the simplest. You must set up a permanent HTTP server to obtain the status of your service (health check).
The address is not very important (e.g., /health).

#### 1.2 Docker Container (3.5 points)
You want to containerize your implementation for easier deployment. An exemple Dockerfile is provided.

Don't forget to:
- Allow opening the necessary ports for your server to work
- Separate your Dockerfile into several sections (compilation, execution, ...)
- Use secure and small base images

### 2. Deployment (6 points)
This section describes the steps necessary to deploy your container from the previous section to your cluster.

#### 2.1 Helm (2 points)
You must create Helm charts to deploy your container to a Kubernetes cluster.

The charts should allow you to:
- Deploy your service from the ACR (Azure Container Registry) provided to you (the image will be push during the next step).
- Add a Kubernetes service to access your pod.
- Add an Ingress to access your service from outside and thus interact with it.
- Add a Service to allow the other cluster (the one with the AI) to access yours.

The other cluster will try to access yours at address: 10.30.10.10.
The service allowing access to your pod from the other cluster is similar to the one provided. However, it is advisable to take a look at the following annotations:
```yml
service.beta.kubernetes.io/azure-load-balancer-ipv4: TODO
service.beta.kubernetes.io/azure-load-balancer-internal: TODO
```

An example Helm chart is provided. You can also recreate a chart template with the command `helm create NAME_OF_PROJECT`.

#### 2.2 Azure (2 points)
In this step, you must create an image of your service, push this image to the ACR, and deploy your service using the charts from the previous step.

Here, the goal is not to deploy from Gitlab, but rather from your computer to validate that what you have done so far is functional.

The variables required for deployment are as follows:
- Your Azure username
- Your Azure password
- The name of the ACR
- The name of the Resource Group
- The name of the cluster
- The domain name: team{team number}.dev.cs2024.one

All these steps must be scripted so than can be easily reproducible. You need to refer to the Helm and Azure cli documentation to complete this step.

> An exemple (``deploy-aks.sh``) is provided. You can start from there.

#### 2.3 Gitlab Pipeline (2 points)
Based on the steps from section 2.2, you must automate deployment through a Gitlab pipeline.

The pipeline should allow you to:
- Compile the code.
- Check the code structure (lint). This can be done with Clippy.
- Deploy to the AKS (Azure Kubernetes Service) cluster.

The first two steps are left to you. For the last one, here are some suggestions:
- Generate the Docker image from the pipeline.
- Reuse the script from the previous step to push the image to the ACR and deploy the Helm charts.
- Place the necessary deployment information as environment variables in the Gitlab repository configurations.
- Use Docker-in-Docker (dind). Since Gitlab pipelines are containerized by default, dind is very useful for enabling the use of Docker from the pipeline.

docker-in-docker job exemple :
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

> The image ``brqu/docker-az:latest`` is based on ``docker:24.0.5`` and also contains helm et AZ cli. By using Gitlab pipelines, you won't need to install these tools for each deployment, which will speed up your pipeline.

> To login into the azure client from the pipeline, you will need to use the ``az login`` command with your username and password.

### 3. Access the Jungle (2 points)
In this step, you must access the status page on the jungle prisoners' service. To do this, you must make an HTTP GET request to the following address from your service:
- http://ai.private.dev.cs2024.one/jungle

The data is returned in the body of the response in JSON. The data is composed of a list of ``Step`` where a ``Step`` is defined by the following interface:
```typescript
interface Step {
    name: string;
    status: string;
}
```

You must be able to access the information returned by this request by making a request to your own service.

### 4. Free the Prisoners (6 points)
The objective of this section is to provide the information necessary for the prisoners to free themselves.

#### 4.1 Provide Access (1 point)
In order to accomplish the following steps, it is necessary for the prisoners in the jungle to be able to communicate with your team. To do this, they will make requests through the AI which will be redirected to your cluster.

The requests will be ``POST`` to the following path: ``/router``.

Each request contains a query parameter: ``request``. This parameter indicates the type of request coming from the jungle.
The body of the request contains information serialized in a specific _JSON_ format for each request.

To validate that you are able to receive requests from the jungle, simply listen to the address ``/router`` for requests with the parameter ``?request=status``.
To indicate that the message is well received, simply respond to the request with an error code within the 200 range.

#### 4.2 Weather (1.5 points)
In order to escape from their bunker, the prisoners must have access to weather conditions. Indeed, plants love warn weather, so they will escape when it is colder.

To obtain weather information, the prisoners will make a request to the path ``/router?request=weather``.

The request body will contain the coordinates of the location for which they want to obtain the weather. The payload is in the following JSON format:
```typescript
interface Coords {
    x: number; // latitude
    y: number; // longitude
}
```

You must return the weather information (in response to the request) in the following JSON format:
```typescript
export interface Weather {
    temperature: number; // Celcius
    windSpeed: number; // Km/h
    precipitation: number; // mm
}
```

> To obtain weather information, it is suggested to use the following API: [https://api.open-meteo.com](https://api.open-meteo.com).
> If you use another API, the results will be considered valid if the precipitation and temperature are similar (5mm/cm, 5°C).

#### 4.3 Map (2 points)
In order to escape from their bunker, the prisoners must have access to the map of the jungle. This map takes the form of a Docker container.

Map image: ``brqu/jungle-map``.

This container must be deployed on the same cluster as your container.

> Note: It is strongly recommended to deploy with Helm (see ``helm init``).

Requests from the jungle (to ``/router``) will have the parameter ``request=map``.
The payload of the body will be in the following format:
```typescript
interface MapRequest {
    x: number, // latitude, float
    y: number, // longitude, float
    size: number, // map size, positive integer
}
```

To obtain a map from this container, you can make a `POST` request to `/`.
The parameters are query parameters with the same names as the attributes of the `MapRequest` interface. For example:

```
http://[MAP_CONTAINER_URL]/?x=75.653&y=-45.6534&size=3
```

The container then responds with a ``JSON`` payload formatted as follows:

```typescript
SIZE = ...
interface MapResponse {
    map: number[SIZE][SIZE]; // Matrix of intergers
}
```

This information can be directly returned to the request (on `/router`) since the response format is the same!

If this step is successful, the status page (http://ai.private.dev.cs2024.one/jungle) should be updated after about a minute.

#### 4.4 Provide the prisoners with the door password (1.5 points)
Now that the prisoners have access to the weather and the map, all that remains is to give them the password for the door separating them from the outside world.

Fortunately for them, the passwords chosen by the AI are inspired by real passwords that are present in a well-known [list](https://raw.githubusercontent.com/DavidWittman/wpxmlrpcbrute/master/wordlists/1000-most-common-passwords.txt).

When they try to open the door, a one-time use password is "generated" from this list. The prisoners have found a way to intercept the md5 hash of this password!
However, they do not have sufficient computing power to find the correct password in less than 500ms. So they still need your help.

The hashed passwords will be transmitted encoded in base64 format to `/router?request=door` in a payload in the following format:

```typescript
interface Door {
    hash: string; // base64 encoded md5 hash
}
```

The jungle does not expect a response from this request, just respond with a code in the 200 range.

You must find the password corresponding to the hash in less than 500ms and send it to the jungle.

To send the password to the jungle, you must make a POST request to the following address:
```
http://ai.private.dev.cs2024.one/jungle/unlock?password=UNHASHED_PASSWORD
```

If the password is correctly returned, the jungle's status page should update in about a minute.

### 5. Bonus (0.5 points)

Where is the bunker?

> ?

## Grading and submission

The grading criteria are as follows:

| Critères           | Score /20 |
|--------------------|-----------|
| Starting code      | /5        |
| Deployment         | /6        |
| Access the Jungle  | /2        |
| Free the Prisoners | /6        |
| Solution quality   | /1        |

The first four criteria are detailed in the [Challenges](#challenges) section.

The final criterion will be evaluated based on the overall coherence of the solution and will be assessed according to the following criteria (loss of up to 1 point):
- No linter is used: -0.5.
- No language-specific conventions are respected: -0.5.
- Duplicated code (e.g., not using the deployment script in the pipeline): -0.5.
- Presence of secrets in the code (e.g., password): -0.5.
- Environment-specific values present directly in the code (try to use environment variables): -0.25.

> Note: The evaluation is partially automated, however, all of your work will be reviewed manually.
> If in doubt, indicate your assumptions in the [Comments](#comments) section à the end of this file.

### Submission
- The submission is done through Git, you must submit all your work through this repository
- The last commit pushed to the `main` branch will be graded
- Any commit pushed after the submission deadline will be ignored

## Comments

...
