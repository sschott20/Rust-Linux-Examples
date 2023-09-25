#include "mdriver.h"
#include <stdio.h>
#include <fcntl.h>  /* open */
#include <unistd.h> /* exit */
#include <sys/wait.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#define W 43
#define N 1843
#define INIT_VAL 69

pid_t wpid;
pthread_t threads[W];

pthread_mutex_t lock;
long long buff = INIT_VAL;
int ret_val, file_desc;

void *thread(void *arg)
{
    for (int i = 0; i < N; i++)
    {
        pthread_mutex_lock(&lock);

        lseek(file_desc, 0, SEEK_SET);
        read(file_desc, &buff, 8);
        buff++;
        lseek(file_desc, 0, SEEK_SET);
        write(file_desc, &buff, 8);
        pthread_mutex_unlock(&lock);
    }
    pthread_exit(NULL);
}

int main()
{

    clock_t start, end;

    int status = 0;

    file_desc = open(DEVICE_FILE_NAME, O_RDWR);

    write(file_desc, &buff, 8);
    pthread_mutex_init(&lock, NULL);

    // using pthread_create create N threads
    // each thread will read the value of buff, increment it, and write it back

    for (int i = 0; i < W; i++)
    {
        pthread_create(&threads[i], NULL, thread, NULL);
    }

    // Basic test that fails due to data race
    // for (int i = 0; i < W; i++)
    // {
    //     int f = fork();
    //     if (f == 0)
    //     {
    //         for (int j = 0; j < N; j++)
    //         {
    //             lseek(file_desc, 0, SEEK_SET);
    //             read(file_desc, &buff, 8);
    //             buff++;
    //             lseek(file_desc, 0, SEEK_SET);
    //             write(file_desc, &buff, 8);
    //         }
    //         exit(0);
    //     }
    // }
    for (int i = 0; i < W; i++)
    {
        pthread_join(threads[i], NULL);
    }

    lseek(file_desc, 0, SEEK_SET);
    read(file_desc, &buff, 8);
    close(file_desc);

    printf("final buffer: %lld\n", buff);
    printf("target val: %lld\n", (long long)INIT_VAL + W * N);
    // close(file_desc);
}