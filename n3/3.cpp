#include <iostream>
#include <vector>
#include <string>
#include <cmath>

using namespace std;

const string digits = "123456789";
const int N = 8;

void search(int pos, string expr, long long current_value, long long last_value, int m) {
    if (pos == N) {
        long long total = current_value + last_value;
        if (total == m) {
            cout << expr << endl;
        }
        return;
    }

    int digit = digits[pos + 1] - '0';

    search(pos + 1, expr + digits[pos + 1], current_value, last_value * 10 + (last_value >= 0 ? digit : -digit), m);
    search(pos + 1, expr + "+" + digits[pos + 1], current_value + last_value, digit, m);
    search(pos + 1, expr + "-" + digits[pos + 1], current_value + last_value, -digit, m);
}

int main() {
    int m;
    cout << "Введите число m: ";
    cin >> m;

    int first_digit = digits[0] - '0';
    search(0, string(1, digits[0]), 0, first_digit, m);

    return 0;
}