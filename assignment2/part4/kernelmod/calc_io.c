
#define _IOC_NRBITS 8
#define _IOC_TYPEBITS 8
#define _IOC_SIZEBITS 13
#define _IOC_DIRBITS 3

#define _IOC_NRMASK ((1 << _IOC_NRBITS) - 1)
#define _IOC_TYPEMASK ((1 << _IOC_TYPEBITS) - 1)
#define _IOC_SIZEMASK ((1 << _IOC_SIZEBITS) - 1)
#define _IOC_DIRMASK ((1 << _IOC_DIRBITS) - 1)

#define _IOC_NRSHIFT 0
#define _IOC_TYPESHIFT (_IOC_NRSHIFT + _IOC_NRBITS)
#define _IOC_SIZESHIFT (_IOC_TYPESHIFT + _IOC_TYPEBITS)
#define _IOC_DIRSHIFT (_IOC_SIZESHIFT + _IOC_SIZEBITS)

/*
 * Direction bits _IOC_NONE could be 0, but OSF/1 gives it a bit.
 * And this turns out useful to catch old ioctl numbers in header
 * files for us.
 */
#define _IOC_NONE 1U
#define _IOC_READ 2U
#define _IOC_WRITE 4U

#define _IOC(dir, type, nr, size)                \
    ((unsigned int)(((dir) << _IOC_DIRSHIFT) |   \
                    ((type) << _IOC_TYPESHIFT) | \
                    ((nr) << _IOC_NRSHIFT) |     \
                    ((size) << _IOC_SIZESHIFT)))

/* used to create numbers */
#define _IO(type, nr) _IOC(_IOC_NONE, (type), (nr), 0)
#define _IOR(type, nr, size) _IOC(_IOC_READ, (type), (nr), sizeof(size))
#define _IOW(type, nr, size) _IOC(_IOC_WRITE, (type), (nr), sizeof(size))
#define _IOWR(type, nr, size) _IOC(_IOC_READ | _IOC_WRITE, (type), (nr), sizeof(size))

/* used to decode them.. */
#define _IOC_DIR(nr) (((nr) >> _IOC_DIRSHIFT) & _IOC_DIRMASK)
#define _IOC_TYPE(nr) (((nr) >> _IOC_TYPESHIFT) & _IOC_TYPEMASK)
#define _IOC_NR(nr) (((nr) >> _IOC_NRSHIFT) & _IOC_NRMASK)
#define _IOC_SIZE(nr) (((nr) >> _IOC_SIZESHIFT) & _IOC_SIZEMASK)

/* ...and for the drivers/sound files... */

#define IOC_IN (_IOC_WRITE << _IOC_DIRSHIFT)
#define IOC_OUT (_IOC_READ << _IOC_DIRSHIFT)
#define IOC_INOUT ((_IOC_WRITE | _IOC_READ) << _IOC_DIRSHIFT)
#define IOCSIZE_MASK (_IOC_SIZEMASK << _IOC_SIZESHIFT)
#define IOCSIZE_SHIFT (_IOC_SIZESHIFT)

#include <stdio.h>
#include <stdlib.h> /* atoi */

int main(int argc, char *argv[])
{
    // get command line arg as int
    if (argc != 3)
    {
        printf("Usage: ./get_sizes <arg> <size>\n");
        return 1;
    }

    int sz = atoi(argv[2]);
    char data_size[sz];
    int arg = atoi(argv[1]);

    printf("%d\n", _IOR('V', arg, data_size));
}