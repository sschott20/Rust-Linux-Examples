#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/ioctl.h>
#include <linux/device.h>
#include <linux/mm.h>
#include <linux/uaccess.h>
#include <linux/fs.h>
#include "mdriver.h"
#include <linux/proc_fs.h>

#define DEVICE_NAME "mdriver"

struct class *my_class;
dev_t my_dev;

static void *base_ptr;
static void *offset_ptr;
static int device_open = 0;
static size_t total_size;

static ssize_t mdriver_read(struct file *file, char *buffer, size_t length, loff_t *offset);
static ssize_t mdriver_write(struct file *file, const char *buffer, size_t length, loff_t *offset);
static int mdriver_open(struct inode *inode, struct file *file);
static int mdriver_close(struct inode *inode, struct file *file);
static loff_t mdriver_llseek(struct file *file, loff_t off, int whence);

static const struct file_operations fops =
    {
        llseek : mdriver_llseek,
        read : mdriver_read,
        write : mdriver_write,
        open : mdriver_open,
        release : mdriver_close,
    };

static loff_t mdriver_llseek(struct file *file, loff_t off, int whence)
{
    switch (whence)
    {
    case SEEK_SET:
        offset_ptr = base_ptr + off;
        break;

    case SEEK_CUR:
        offset_ptr += off;
        break;

    case SEEK_END:
        offset_ptr = base_ptr + total_size + off;
        break;
    default:
        break;
    }
    return (loff_t)offset_ptr;
}

static ssize_t mdriver_read(struct file *file, char *buffer, size_t length, loff_t *offset)
{
    pr_info("device_read: (%p,%p,%ld) \n", file, buffer, length);

    if ((uintptr_t)length + (uintptr_t)offset_ptr - (uintptr_t)base_ptr > (uintptr_t)total_size)
    {
        // print value of both parts

        pr_info("Error: read too large\n");
        return -EINVAL;
    }

    if (copy_to_user(buffer, offset_ptr, length) > 0)
    {
        pr_info("Error: copy_from_user failed\n");
        return -EFAULT;
    }

    offset_ptr += length;

    *offset += length;
    return length;
    return 0;
}

static ssize_t mdriver_write(struct file *file, const char *buffer, size_t length, loff_t *offset)
{
    pr_info("device_write (%p,%s,%ld) \n", file, buffer, length);

    if ((uintptr_t)length + (uintptr_t)offset_ptr - (uintptr_t)base_ptr > (uintptr_t)total_size)
    {
        pr_info("Error: write too large\n");
        return -EINVAL;
    }

    if (copy_from_user(offset_ptr, buffer, length) > 0)
    {
        pr_info("Error: copy_from_user failed\n");
        return -EFAULT;
    }

    offset_ptr += length;

    *offset += length;
    return length;
}

static int mdriver_open(struct inode *inode, struct file *file)
{
    pr_info("mdriver_open: successful\n");

    if (device_open)
    {
        pr_info("mdriver_open: device busy\n");
        return -EBUSY;
    }

    device_open++;
    return 0;
}

static int mdriver_close(struct inode *inode, struct file *file)
{
    pr_info("mdriver_close: successful\n");

    device_open--;
    return 0;
}

static int mdriver_init(void)
{
    int ret_val;

    pr_info("Loading mdriver\n");

    ret_val = register_chrdev(MAJOR_NUM, DEVICE_NAME, &fops);
    // proc_file_entry = proc_create(DEVICE_NAME, 0, NULL, &fops);
    if (ret_val < 0)
    {
        pr_info("Error registering device driver\n");
        return ret_val;
    }

    printk("mknod %s c %d 0\n", DEVICE_FILE_NAME, MAJOR_NUM);

    // my_class = class_create(THIS_MODULE, "my_class");
    // device_create(my_class, NULL, my_dev, NULL, "mycdrv");

    base_ptr = (void *)__get_free_pages(GFP_KERNEL, NUMPAGES);
    if (base_ptr == NULL)
    {
        pr_info("Error allocating memory\n");
        return -1;
    }

    offset_ptr = base_ptr;
    total_size = PAGE_SIZE * (1 << NUMPAGES);

    // memcpy(base_ptr, , 10);

    pr_info("Allocated %d pages (%ld bytes)\n", (1 << NUMPAGES), total_size);
    pr_info("Finished loading mdriver\n");
    return 0;
}

static void __exit mdriver_exit(void)
{
    for (int i = 0; i < 40; i++)
    {
        pr_info("%c", ((char *)base_ptr)[i]);
    }

    // device_destroy(my_class, my_dev);
    // class_destroy(my_class);

    free_pages((unsigned long)base_ptr, NUMPAGES);
    unregister_chrdev(MAJOR_NUM, DEVICE_NAME);
    pr_info("mdriver unloaded\n");
}

module_init(mdriver_init);
module_exit(mdriver_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Alex Schott");