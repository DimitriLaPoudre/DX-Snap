NAME = ./DX-Snap/DX-Snap

SRC = $(wildcard src/*.cpp)

OBJ = $(SRC:.cpp=.o)

CXX = g++

CXXFLAGS = -Wall -Wextra -Werror -std=c++20 -I./include

LDFLAGS = -lGL -lglfw -lGLEW -lX11 -lXrandr -lXi

all: $(NAME)

$(NAME): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $(NAME) $(OBJ) $(LDFLAGS)

clean:
	rm -f $(OBJ)

fclean: clean

re: fclean all