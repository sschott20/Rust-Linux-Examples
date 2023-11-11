typedef unsigned int u32;
typedef unsigned char u8;

u32 create_address(u8 *ip)
{
    u32 addr = 0;
    int i;

    for (i = 0; i < 4; i++)
    {
        addr += ip[i];
        if (i == 3)
            break;
        addr <<= 8;
    }
    return addr;
}
int main(int argc, char *argv[])
{
    unsigned char destip[5] = {127, 0, 0, 1, '\0'};
    int destaddr = create_address(destip);
    printf("destaddr: %d\n", destaddr);
}