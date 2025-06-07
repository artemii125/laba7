#include <iostream>
#include <vector>
#include <string>
using namespace std;

int factorial(int n) {
    int res = 1;
    for (int i = 2; i <= n; ++i)
        res *= i;
    return res;
}

string getPermutation(int n, int k) {
    vector<int> numbers;

    for (int i = 1; i <= n; ++i)
        numbers.push_back(i);

    k -= 1;
    string result = "";
    for (int i = n; i > 0; --i) {
        int fact = factorial(i - 1);
        int index = k / fact;
        
        result += to_string(numbers[index]);
        numbers.erase(numbers.begin() + index);
        k %= fact;
    }
    return result;
}

int main() {
    int n, k;
    cout << "Введите n и k по ограничению 1 <= n <= 9 и 1 <= k <= n!: ";
    cin >> n >> k;

    if ((n <= 0 || n > 9) || (k <= 0 || k > factorial(n))) return 0;
    
    cout << getPermutation(n, k) << endl;

    return 0;
}
