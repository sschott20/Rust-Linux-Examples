ffmpeg -f v4l2              \
       -input_format mjpeg  \
       -framerate 1        \
       -video_size 640x360 \
       -i /dev/video0       \
       -pix_fmt yuyv422     \
       -f v4l2 /dev/video2  

