class Animal
{
public:
    int age;
    char gender[20];

    bool isMammal(){};
    void mate(){};
};

class Duck : Animal
{
public:
    char beakColor[20];

    void swim(){};
    void quack(){};
};

class Fish : Animal
{
private:
    int sizeInFeet;

    bool canEat(){};
};

class Zebra : Animal
{
private:
    bool is_wild;

    void run(){};
};
