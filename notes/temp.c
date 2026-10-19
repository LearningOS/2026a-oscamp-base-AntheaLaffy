void *my_memmove(char* dst[], char* src[], unsigned int n) {
    unsigned int i = 0;
    if (dst <= src) {
        while (i < n) {
            *(dst + i) = *(src + i);
			i++;
        }
    } else {
        while (i < n) {
            *(dst + n - 1 - i) = *(src + n - 1 - i);
			i++;
        }
    }
    return dst;
}
