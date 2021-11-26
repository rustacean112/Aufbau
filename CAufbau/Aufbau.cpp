#include <iostream>
#include <string>
#include <map>
#include <list>
using namespace std;

int main()
{
    map<char, int> mlDict =
        {
            {'s', 0},
            {'p', 1},
            {'d', 2},
            {'f', 3}
        };

    map<int, string> exceptions =
        {
            {24, "1s^2 2s^2 2p^6 3s^2 3p^6 4s^1 3d^5"},
            {29, "1s^2 2s^2 2p^6 3s^2 3p^6 4s^1 3d^10"}
        };

    string orbitals[] = {"1s^", "2s^", "2p^", "3s^", "3p^", "4s^", "3d^", "4p^", "5s^", "4d^", "5p^", "6s^", "4f^", "5d^", "6p^", "7s^", "5f^", "6d^", "7p^"};

    // Variables
    string configuration = "";
    int electronNum;
    int configurationStep = 0;
    int e = 0;
    bool loop = true;

    // Result text
    string result = "Configuration of Element that you selected is: ";

    std::cout << "Say the electron number: ";
    std::cin >> electronNum;

    if (electronNum == 29 || electronNum == 24) {
        std::cout << result << exceptions[electronNum];
    }else{
        while (loop)
        {
            char l = orbitals[configurationStep][1];
            int ml = mlDict[l];
            int mln = (2*ml+1)*2;
            if (electronNum > mln){
                e = mln;
                string orbital = orbitals[configurationStep];
                configuration += orbital + std::to_string(e) + " ";
                configurationStep += 1;
                electronNum -= mln;
            }else if (electronNum == mln){
                e = mln;
                string orbital = orbitals[configurationStep];
                configuration += orbital + std::to_string(e) + "";
                std::cout << result + configuration + "\n";
                break;
            }else if (electronNum < mln){
                e = electronNum;
                string orbital = orbitals[configurationStep];
                configuration += orbital + std::to_string(e) + " ";
                std::cout << result + configuration + "\n";
                break;
            };
        };

    };
    return 0;
}
