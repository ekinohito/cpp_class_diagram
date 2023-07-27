class ClassA
{
    int rich_method()
    {
        int a = 2;
        int b = 3;

        b = b * 5 - 6 * a;

        int c = a + b;
        bool d = c > 4;

        while (a > 0)
        {
            a--;
        }

        do
        {
            break;
        } while (true);

        for (int i = 1; i < 8; ++i)
        {
            c++;
        }

        if (a < c)
        {
            b += 2;
        }
        else if (a == c)
        {
            b += 1;
        }
        else
        {
            b -= 1;
        }

        switch (a)
        {
        case 1:
            a++;
            break;
        case 2:
            b++;
        default:
            c--;
            break;
        }

        return c;
    }
};