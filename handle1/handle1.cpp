#include <iostream>
using namespace std;
#include <handle1.h>
extern "C"{
    int handle1(int v){
        cout << "handle1" << endl;
        cout << v << endl;

        return v;
    }
}