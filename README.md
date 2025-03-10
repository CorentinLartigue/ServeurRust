# Serveur HTTP Rust Simple

Ce projet implémente un serveur HTTP basique en Rust qui écoute sur un port spécifié et répond aux requêtes `GET /ping` en renvoyant les en-têtes de la requête sous forme de JSON. 
Pour toutes les autres requêtes, le serveur renvoie une réponse vide avec un code `404 Not Found`.

## Prérequis

Avant de pouvoir lancer ce projet, assurez-vous d'avoir les éléments suivants installés :

- **Rust** : Si vous n'avez pas encore installé Rust, vous pouvez le télécharger et l'installer à partir de [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- **Visual Studio ou Build Tools pour Windows** (si vous utilisez Windows) :
      Pour pouvoir compiler le projet sous Windows, vous devez installer **Visual Studio** ou les **Build Tools** avec le support C++ et le SDK Windows.
      Plus d'informations ici : [Installation](https://visualstudio.microsoft.com/fr/visual-cpp-build-tools/).

## Installation

1. Clonez le repository du projet :
   ```bash
      git clone https://github.com/CorentinLartigue/ServeurRust.git
   ```
   
2. Se déplacer dans le dossier du projet :
   ```bash
      cd ServeurRust
   ```
   
3. Installez les dépendances (si nécessaire) :
   ```bash
      cargo build
   ```
   
## Configuration du port d'écoute

Le serveur écoute sur le port spécifié dans la variable d'environnement PING_LISTEN_PORT. Si cette variable n'est pas définie, le port par défaut est 8080.

Si vous souhaitez changer le port, vous pouvez définir la variable d'environnement PING_LISTEN_PORT avant d'exécuter le programme. Par exemple, pour démarrer le serveur sur le port 9090.

## Lancer le projet

Pour démarrer le serveur, exécutez la commande suivante dans le terminal:
  ```bash
      cargo run
  ```
Le serveur affichera un message indiquant qu'il est démarré et qu'il écoute à l'adresse 127.0.0.1:8080.

## Fonctionnement du serveur
Le serveur écoute les requêtes GET /ping. Si une telle requête est reçue, le serveur répond par une réponse HTTP 200 OK contenant un objet JSON avec les en-têtes de la requête. 
Si la requête ne correspond pas, une réponse 404 Not Found est renvoyée.


### Test de la fonctionnalité
Pour tester le serveur avec la requête GET /ping, vous pouvez utiliser curl ou un navigateur :
```bash
  curl http://127.0.0.1:8080/ping
```

Pour toute autre requête, comme GET /test, le serveur renverra une réponse vide avec le code 404 :
```bash
curl http://127.0.0.1:8080/test
```

## Auteur
Lartigue Corentin
