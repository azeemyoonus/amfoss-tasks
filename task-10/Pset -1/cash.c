#include <stdio.h>
#include <math.h>
#include <cs50.h>

int main()
{
    int ans,owed;

    do
    {
        float d_owed = get_float("Change owed: \n");
        owed = round(d_owed * 100);
    }
    while (owed <= 0);

    int quarters = owed / 25;
    int dimes = (owed % 25) / 10;
    int nickels = ((owed % 25) % 10) / 5;
    int pennies = ((owed % 25) % 10) % 5;
    ans=quarters+dimes+nickels+pennes;

    printf("%d",ans);
}