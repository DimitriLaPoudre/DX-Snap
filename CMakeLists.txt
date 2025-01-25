cmake_minimum_required(VERSION 3.10)
project(DX-Snap VERSION 0.0)

# Définir les options de compilation
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Répertoires de sources et headers
include_directories(${CMAKE_SOURCE_DIR}/include)
file(GLOB SOURCES ${CMAKE_SOURCE_DIR}/src/*.cpp)

# Ajouter les bibliothèques (si nécessaires)
link_directories(${CMAKE_SOURCE_DIR}/lib)

# Créer l'exécutable principal
add_executable(DX-Snap ${SOURCES})

# Ajouter des bibliothèques spécifiques si nécessaires
# target_link_libraries(mon_app libnomdelib.so)

# Installer l'exécutable et les fichiers associés
install(TARGETS DX-Snap RUNTIME DESTINATION usr/local/bin)
install(DIRECTORY ${CMAKE_SOURCE_DIR}/docs/ DESTINATION usr/share/doc/Dx-Snap)

# Configuration pour le paquet .deb avec CPack
set(CPACK_PACKAGE_NAME "${PROJECT_NAME}")
set(CPACK_PACKAGE_VERSION "${PROJECT_VERSION}")
set(CPACK_PACKAGE_CONTACT "TonNom <tonemail@example.com>")
set(CPACK_PACKAGE_DESCRIPTION "tmtc")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "DX-Snap > Marvel Snap")
set(CPACK_PACKAGE_VENDOR "Drysfa-Xaar & Co")
set(CPACK_PACKAGE_FILE_NAME "${CPACK_PACKAGE_NAME}-${CPACK_PACKAGE_VERSION}-${CMAKE_SYSTEM_PROCESSOR}")
set(CPACK_DEBIAN_PACKAGE_DEPENDS "libc6, libstdc++6")
set(CPACK_DEBIAN_PACKAGE_PRIORITY "optional")
set(CPACK_DEBIAN_PACKAGE_SECTION "net")
set(CPACK_GENERATOR "DEB")

# Inclure CPack
include(CPack)
