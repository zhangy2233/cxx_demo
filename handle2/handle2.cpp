#include <iostream>
using namespace std;
#include <handle2.h>
#include <handle1.h>
extern "C"{
    int handle2(int v){
        cout << "handl2" << endl;
        handle1(v);
        return v;
    }
}