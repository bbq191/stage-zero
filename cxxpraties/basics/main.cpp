#include <iostream>

struct car
{
  int m_price;
  void run()
  {
    this->m_price = 3;
    // std::cout << "price " << m_price << " car run away" << std::endl;
  }
};

int main(int argc, const char *argv[])
{
  car a;
  a.m_price = 1;
  a.run();

  car b;
  b.m_price = 2;
  b.run();

  std::cout << sizeof(a) << sizeof(b) << std::endl;
  return 0;
}