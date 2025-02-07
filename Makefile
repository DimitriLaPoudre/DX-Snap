NAME = ./DX-Snap/DX-Snap

SRC = $(wildcard src/*.cpp)

OBJ = $(SRC:.cpp=.o)

CXX = g++

CXXFLAGS = -Wall -Wextra -Werror -std=c++20 -I./include

LDFLAGS = -static-libgcc -static-libstdc++ -lGL -lglfw /usr/lib/x86_64-linux-gnu/libGLEW.a /usr/lib/x86_64-linux-gnu/libXrandr.a /usr/lib/x86_64-linux-gnu/libXi.a /usr/lib/x86_64-linux-gnu/libfreetype.a -lcurl

all: $(NAME)

$(NAME): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $(NAME) $(OBJ) $(LDFLAGS)

clean:
	rm -f $(OBJ)

fclean: clean
	rm -f $(NAME)

re: fclean all