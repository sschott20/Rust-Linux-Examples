CONFIG_MODULE_SIG=n

obj-m += mdriver.o

all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules
	gcc -o mdriver_test mdriver_test.c -I -Wall -Werror 

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean