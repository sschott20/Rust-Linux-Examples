/*  chardev.c - Create an input/output character device
 */

#include <linux/kernel.h> /* We're doing kernel work */
#include <linux/module.h> /* Specifically, a module */

/* Deal with CONFIG_MODVERSIONS */
#if CONFIG_MODVERSIONS == 1
#define MODVERSIONS
#include <linux/modversions.h>
#endif

/* For character devices */

/* The character device definitions are here */
#include <linux/fs.h>

/* A wrapper which does next to nothing at
 * at present, but may help for compatibility
 * with future versions of Linux */
#include <linux/wrapper.h>

/* Our own ioctl numbers */
#include "chardev.h"

/* In 2.2.3 /usr/include/linux/version.h includes a
 * macro for this, but 2.0.35 doesn't - so I add it
 * here if necessary. */
#ifndef KERNEL_VERSION
#define KERNEL_VERSION(a, b, c) ((a) * 65536 + (b) * 256 + (c))
#endif

#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
#include <asm/uaccess.h> /* for get_user and put_user */
#endif

#define SUCCESS 0

/* Device Declarations ******************************** */

/* The name for our device, as it will appear in
 * /proc/devices */
#define DEVICE_NAME "char_dev"

/* The maximum length of the message for the device */
#define BUF_LEN 80

/* Is the device open right now? Used to prevent
 * concurent access into the same device */
static int Device_Open = 0;

/* The message the device will give when asked */
static char Message[BUF_LEN];

/* How far did the process reading the message get?
 * Useful if the message is larger than the size of the
 * buffer we get to fill in device_read. */
static char *Message_Ptr;

/* This function is called whenever a process attempts
 * to open the device file */
static int device_open(struct inode *inode,
                       struct file *file)
{
#ifdef DEBUG
    printk("device_open(%p)\n", file);
#endif

    /* We don't want to talk to two processes at the
     * same time */
    if (Device_Open)
        return -EBUSY;

    /* If this was a process, we would have had to be
     * more careful here, because one process might have
     * checked Device_Open right before the other one
     * tried to increment it. However, we're in the
     * kernel, so we're protected against context switches.
     *
     * This is NOT the right attitude to take, because we
     * might be running on an SMP box, but we'll deal with
     * SMP in a later chapter.
     */

    Device_Open++;

    /* Initialize the message */
    Message_Ptr = Message;

    MOD_INC_USE_COUNT;

    return SUCCESS;
}

/* This function is called when a process closes the
 * device file. It doesn't have a return value because
 * it cannot fail. Regardless of what else happens, you
 * should always be able to close a device (in 2.0, a 2.2
 * device file could be impossible to close).
 */
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
static int device_release(struct inode *inode,
                          struct file *file)
#else
static void device_release(struct inode *inode,
                           struct file *file)
#endif
{
#ifdef DEBUG
    printk("device_release(%p,%p)\n", inode, file);
#endif

    /* We're now ready for our next caller */
    Device_Open--;

    MOD_DEC_USE_COUNT;

#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
    return 0;
#endif
}

/* This function is called whenever a process which
 * has already opened the device file attempts to
 * read from it. */
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
static ssize_t device_read(
    struct file *file,
    char *buffer,   /* The buffer to fill with the data */
    size_t length,  /* The length of the buffer */
    loff_t *offset) /* offset to the file */
#else
static int device_read(
    struct inode *inode,
    struct file *file,
    char *buffer, /* The buffer to fill with the data */
    int length)   /* The length of the buffer
                   * (mustn't write beyond that!) */
#endif
{
    /* Number of bytes actually written to the buffer */
    int bytes_read = 0;

#ifdef DEBUG
    printk("device_read(%p,%p,%d)\n", file, buffer, length);
#endif

    /* If we're at the end of the message, return 0
     * (which signifies end of file) */
    if (*Message_Ptr == 0)
        return 0;

    /* Actually put the data into the buffer */
    while (length && *Message_Ptr)
    {

        /* Because the buffer is in the user data segment,
         * not the kernel data segment, assignment wouldn't
         * work. Instead, we have to use put_user which
         * copies data from the kernel data segment to the
         * user data segment. */
        put_user(*(Message_Ptr++), buffer++);
        length--;
        bytes_read++;
    }

#ifdef DEBUG
    printk("Read %d bytes, %d left\n", bytes_read, length);
#endif

    /* Read functions are supposed to return the number
     * of bytes actually inserted into the buffer */
    return bytes_read;
}

/* This function is called when somebody tries to
 * write into our device file. */
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
static ssize_t device_write(struct file *file,
                            const char *buffer,
                            size_t length,
                            loff_t *offset)
#else
static int device_write(struct inode *inode,
                        struct file *file,
                        const char *buffer,
                        int length)
#endif
{
    int i;

#ifdef DEBUG
    printk("device_write(%p,%s,%d)",
           file, buffer, length);
#endif

    for (i = 0; i < length && i < BUF_LEN; i++)
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
        get_user(Message[i], buffer + i);
#else
        Message[i] = get_user(buffer + i);
#endif

    Message_Ptr = Message;

    /* Again, return the number of input characters used */
    return i;
}

/* This function is called whenever a process tries to
 * do an ioctl on our device file. We get two extra
 * parameters (additional to the inode and file
 * structures, which all device functions get): the number
 * of the ioctl called and the parameter given to the
 * ioctl function.
 *
 * If the ioctl is write or read/write (meaning output
 * is returned to the calling process), the ioctl call
 * returns the output of this function.
 */
int device_ioctl(
    struct inode *inode,
    struct file *file,
    unsigned int ioctl_num,    /* The number of the ioctl */
    unsigned long ioctl_param) /* The parameter to it */
{
    int i;
    char *temp;
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
    char ch;
#endif

    /* Switch according to the ioctl called */
    switch (ioctl_num)
    {
    case IOCTL_SET_MSG:
        /* Receive a pointer to a message (in user space)
         * and set that to be the device's message. */

        /* Get the parameter given to ioctl by the process */
        temp = (char *)ioctl_param;

        /* Find the length of the message */
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
        get_user(ch, temp);
        for (i = 0; ch && i < BUF_LEN; i++, temp++)
            get_user(ch, temp);
#else
        for (i = 0; get_user(temp) && i < BUF_LEN; i++, temp++)
            ;
#endif

            /* Don't reinvent the wheel - call device_write */
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
        device_write(file, (char *)ioctl_param, i, 0);
#else
        device_write(inode, file, (char *)ioctl_param, i);
#endif
        break;

    case IOCTL_GET_MSG:
        /* Give the current message to the calling
         * process - the parameter we got is a pointer,
         * fill it. */
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
        i = device_read(file, (char *)ioctl_param, 99, 0);
#else
        i = device_read(inode, file, (char *)ioctl_param, 99);
#endif
        /* Warning - we assume here the buffer length is
         * 100. If it's less than that we might overflow
         * the buffer, causing the process to core dump.
         *
         * The reason we only allow up to 99 characters is
         * that the NULL which terminates the string also
         * needs room. */

        /* Put a zero at the end of the buffer, so it
         * will be properly terminated */
        put_user('\0', (char *)ioctl_param + i);
        break;

    case IOCTL_GET_NTH_BYTE:
        /* This ioctl is both input (ioctl_param) and
         * output (the return value of this function) */
        return Message[ioctl_param];
        break;
    }

    return SUCCESS;
}

/* Module Declarations *************************** */

/* This structure will hold the functions to be called
 * when a process does something to the device we
 * created. Since a pointer to this structure is kept in
 * the devices table, it can't be local to
 * init_module. NULL is for unimplemented functions. */
struct file_operations Fops = {
    NULL, /* seek */
    device_read,
    device_write,
    NULL,         /* readdir */
    NULL,         /* select */
    device_ioctl, /* ioctl */
    NULL,         /* mmap */
    device_open,
#if LINUX_VERSION_CODE >= KERNEL_VERSION(2, 2, 0)
    NULL, /* flush */
#endif
    device_release /* a.k.a. close */
};

/* Initialize the module - Register the character device */
int init_module()
{
    int ret_val;

    /* Register the character device (atleast try) */
    ret_val = module_register_chrdev(MAJOR_NUM,
                                     DEVICE_NAME,
                                     &Fops);

    /* Negative values signify an error */
    if (ret_val < 0)
    {
        printk("%s failed with %d\n",
               "Sorry, registering the character device ",
               ret_val);
        return ret_val;
    }

    printk("%s The major device number is %d.\n",
           "Registeration is a success",
           MAJOR_NUM);
    printk("If you want to talk to the device driver,\n");
    printk("you'll have to create a device file. \n");
    printk("We suggest you use:\n");
    printk("mknod %s c %d 0\n", DEVICE_FILE_NAME,
           MAJOR_NUM);
    printk("The device file name is important, because\n");
    printk("the ioctl program assumes that's the\n");
    printk("file you'll use.\n");

    return 0;
}

/* Cleanup - unregister the appropriate file from /proc */
void cleanup_module()
{
    int ret;

    /* Unregister the device */
    ret = module_unregister_chrdev(MAJOR_NUM, DEVICE_NAME);

    /* If there's an error, report it */
    if (ret < 0)
        printk("Error in module_unregister_chrdev: %d\n", ret);
}