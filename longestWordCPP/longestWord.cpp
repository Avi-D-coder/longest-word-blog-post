#include <fstream>
#include <sstream>
#include <string>
using namespace std;

const

    int
    main(int argc, char const *argv[]) {
  ifstream file("./words.txt");
  stringstream buffer;
  buffer << file.rdbuf();
  string word;

  while (getline(buffer, word)) {
  }

  return 0;
}
