//
// Created by Vinci Xu on 11/21/23.
//
#include <iostream>

class Person
{
public:
    int m_id;
    int m_age;
    int m_height;

    void display()
    {
        std::cout << "id: " << m_id << " age: " << m_age << " height: " << m_height << std::endl;
    }
};

int main(int argc, char *argv[])
{
    Person p;
    p.m_id = 10;
    p.m_age = 20;
    p.m_height = 30;
    p.display();

    // eax => &p.m_age = &p + 4
    Person *person = (Person *)&p.m_age;
    // mov eax+0 0x0 == mov [&p+4+0] 0x0
    person->m_id = 0;
    // mov eax+4 0x1 == mov [&p+4+4] 0x1
    person->m_age = 1;
    p.display(); // 传人对象 p 的地址值

    person->display(); // 传入的是 person 指针的地址，而指针指向的是对象 p 的 m_age 地址
}
