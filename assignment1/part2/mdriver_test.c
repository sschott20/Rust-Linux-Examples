#include "mdriver.h"
#include <stdio.h>
#include <fcntl.h>  /* open */
#include <unistd.h> /* exit */
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main()
{
    clock_t start, end;
    int ret_val, file_desc;
    char buff[512000];

    file_desc = open(DEVICE_FILE_NAME, O_RDWR);

    if (file_desc < 0)
    {
        printf("Can't open device file: %s\n", DEVICE_FILE_NAME);
        return -1;
    }

    long bytes_list[5] = {1, 64, 1000, 64000, 512000};
    long bytes = 1;
    for (int i = 0; i <= 4; i++)
    {
        bytes = bytes_list[i];

        printf("Reading %ld bytes from the device...\n", bytes);
        for (int i = 0; i < 10; i++)
        {
            lseek(file_desc, 0, SEEK_SET);
            start = clock();
            read(file_desc, buff, bytes);
            end = clock();
            double time_taken = ((double)(end - start)) / CLOCKS_PER_SEC;
            printf("%f\n", time_taken);
        }

        printf("Writing %ld bytes to the device...\n", bytes);
        for (int i = 0; i < 10; i++)
        {
            lseek(file_desc, 0, SEEK_SET);
            start = clock();
            write(file_desc, buff, bytes);
            end = clock();
            double time_taken = ((double)(end - start)) / CLOCKS_PER_SEC;
            printf("%f\n", time_taken);
        }
    }
    close(file_desc);
}