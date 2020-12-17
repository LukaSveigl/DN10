#include <iostream>
#include <vector>
#include <tuple>
#include <string>


void print() {std::cout << std::endl;} // Stop condition

template <typename H, typename... T> //Template where H is list head and ... is type list
void print(H v, T... t) { // T... t means a list of arguments
    std::cout << v << " ";
    print(t...); //Expand rest of list
}

int main(){
    print("Testing", "Variadic", "Templates", 42, 3.14, 'c', true);
    
    std::tuple<int, int, int, std::string, std::string> values {33, 23, 154, "Testing tuples", "Hope it works"};

    print(std::get<0>(values), std::get<1>(values), std::get<2>(values), std::get<3>(values), std::get<4>(values));

    return 0;
}

