#include <stdio.h>


int main(){
    int s[100];
    s[55] = 955;

    int a;
    asm volatile("unimp");
    return 0;
}