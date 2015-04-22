GPP   = /usr/bin/g++
FLAGS = -std=c++14 -W -Wall -Wctor-dtor-privacy -Wold-style-cast -Woverloaded-virtual

all: primitives.o vector_list.o

primitives.o: primitives.cpp 
	$(GPP) -c $(FLAGS) -Wno-unused-variable primitives.cpp

vector_list.o: vector_list.cpp 
	$(GPP) -c $(FLAGS) vector_list.cpp

clean:
	rm *~ *.o
