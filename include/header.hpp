#ifndef HEADER_HPP
    #define HEADER_HPP

#include <iostream>
#include <GL/glew.h>
#include <GL/gl.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#ifdef _WIN32
    #define OS "Windows"
#elif __linux__
    #define OS "Linux"
#else
    #define OS "Inconnu"
#endif

#endif