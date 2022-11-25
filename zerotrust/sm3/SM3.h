class SM3
{
private:
    unsigned int T(int j);
    unsigned int FF(unsigned int x, unsigned int y, unsigned int z, int j);
    unsigned int GG(unsigned int x, unsigned int y, unsigned int z, int j);
    unsigned int P0(unsigned int X);
    unsigned int P1(unsigned int X);
    unsigned int ROTL(unsigned int x, int n);
    void getbytes(unsigned char *bytes, unsigned long long l);
    unsigned char *padding(unsigned char *input, unsigned long long *len);
    void expend(unsigned char *m, unsigned int *w);

public:
    void doFinal(unsigned char *msg, unsigned char *res);
};