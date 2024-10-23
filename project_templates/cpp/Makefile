all: clean run

CC = clang++
PROJECT_NAME = PROJECT NAME
COMPILER_FLAGS = -Wall -w
INCLUDE_FLAGS = -Iinclude
LINKER_FLAGS = -Llib
SRC = src/main.cpp
ARGS = 

compile:
	$(CC) $(SRC) $(COMPILER_FLAGS) -o bin/$(ASSEMBLY).exe $(INCLUDE_FLAGS) $(LINKER_FLAGS)

game: compile
	mkdir build
	cp bin/$(PROJECT_NAME).exe build

run: build
	@echo "build/$(PROJECT_NAME).exe running..."
	@build/$(PROJECT_NAME).exe $(ARGS)

clear:
	-rm -rf build
	-rm bin/*