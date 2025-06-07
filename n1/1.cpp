#include <iostream>
#include <cmath>
#include <string>
using namespace std;

void cantor(string &line, int left, int right, int n) {
    if (n == 0) {
        return;
    } else {
        int third = (right - left) / 3;

        for (int i = left + third; i < left + 2 * third; i++) line[i] = ' ';

        cantor(line, left, left + third, n - 1);
        cantor(line, left + 2 * third, right, n - 1);
    }
}

int main() {
    int n;
    cout << "Введите количество шагов: ";
    cin >> n;

    int length = pow(3,n);
    for (int i = 0; i < n; ++i){
        string line(length, '-');
        cantor(line, 0, length, i);
        cout << line << endl;
    }
    return 0;
}
