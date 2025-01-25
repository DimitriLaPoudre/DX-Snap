#!/bin/bash

# Vérifie si CMake est installé
command -v cmake >/dev/null 2>&1 || { echo "CMake est requis mais non installé. Abandon."; exit 1; }

# Créer un dossier de build si nécessaire
mkdir -p build
cd build

# Exécuter CMake pour générer les fichiers de configuration
echo "Exécution de CMake..."
cmake ..

# Compiler le projet
echo "Compilation du projet..."
make

# Installer le projet (optionnel, juste pour tester avant le packaging)
echo "Installation du projet..."
make install

# Créer le paquet .deb
echo "Création du paquet .deb..."
make package

#Récuperer le paquet .deb
echo "Récuperation du paquet .deb..."
mv *.deb ../

# Retourner dans le dossier racine et supprimer le dossier de build
echo "Nettoyage..."
cd ..
rm -rf build
