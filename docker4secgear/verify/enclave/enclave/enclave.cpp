#include "Enclave1_t.h"
#include <string.h>
#include <string>
#include <vector>
#include <cstdlib>
#define MAX_LEN 1024

using namespace std;

namespace calc {
	const int L = 16;
	const std::string hexChars = "0123456789abcdef";
	int dec(char ch) {
		if ('0' <= ch && ch <= '9') {
			return ch - '0';
		}
		if ('a' <= ch && ch <= 'f') {
			return ch - 'a' + 10;
		}
		if ('A' <= ch && ch <= 'F') {
			return ch - 'A' + 10;
		}
		return -1;
	}
	std::string hex(unsigned int x) {
		std::string result = "";
		for (int j = 28; j >= 0; j -= 4) {
			result += hexChars[x >> j & 0x0F];
		}
		return result;
	}
	std::string hex(unsigned long long x) {
		std::string result = "";
		for (int j = 60; j >= 0; j -= 4) {
			result += hexChars[x >> j & 0x0F];
		}
		return result;
	}
	std::string bin(unsigned int x) {
		std::string result = "";
		for (int j = 31; j >= 0; j--) {
			result += hexChars[x >> j & 0x01];
		}
		return result;
	}
	std::string bin(unsigned long long x) {
		std::string result = "";
		for (int j = 63; j >= 0; j--) {
			result += hexChars[x >> j & 0x01];
		}
		return result;
	}
	unsigned int rightRotate(unsigned int x, int u) {
		unsigned long long xx = (unsigned long long)x << 32 | x;
		return (unsigned int)((xx >> u)&(unsigned int)-1);
	}
	unsigned int LeftRotate(unsigned int x, int u) {
		u &= 31;
		unsigned long long xx = (unsigned long long)x << 32 | x;
		return (unsigned int)((xx >> (32 - u))&(unsigned int)-1);
	}
	struct bigNum {
		int siz, hexLen;
		std::vector<unsigned long long>num;
		bigNum(int _siz = L) {
			siz = _siz; hexLen = siz << 4;
			num.resize(_siz, 0);
		}
		bigNum(std::string src) {
			int len = src.length(); hexLen = src.length();
			siz = (len + 15) >> 4;
			num.resize(siz, 0);
			for (int i = 0, cnt = 0; i < len; i++, cnt++) {
				num[cnt >> 4] |= (unsigned long long)dec(src[i]) << (((cnt & 0x0F) ^ 15) << 2);
			}
		}
		bigNum FromU32(uint32_t src) {
			bigNum result = bigNum(1);
			result.num[0] = src;
			return result;
		}
		std::string hex() {
			std::string result = "";
			for (int i = 0; i < siz; i++) {
				for (int j = 60; j >= 0; j -= 4) {
					result += hexChars[num[i] >> j & 0x0F];
				}
			}
			result.erase(hexLen);
			return result;
		}
		std::string bin() {
			std::string result = "b";
			for (int i = 0; i < siz; i++) {
				for (int j = 63; j >= 0; j--) {
					result += hexChars[num[i] >> j & 0x01];
				}
			}
			result.erase(hexLen << 2);
			return result;
		}
		bigNum& operator^=(const bigNum &rhs) {
			bigNum result = *this;
			int lmt = siz;
			if (rhs.siz < lmt) lmt = rhs.siz;
			for (int i = 0; i < lmt; i++) {
				result.num[i] ^= rhs.num[i];
			}
			return *this = result;
		}
		friend bigNum operator^(const bigNum& lhs, const bigNum& rhs) {
			if(lhs.siz>=rhs.siz) return bigNum(lhs) ^= rhs;
			else return bigNum(rhs) ^= lhs;
		}
		friend bigNum operator||(const bigNum& lhs, const bigNum& rhs) {
			bigNum result = bigNum(bigNum(lhs).hex() + bigNum(rhs).hex());
			return result;
		}
	};
	
	unsigned int P0(unsigned int x) {
		return x ^ LeftRotate(x, 9) ^ LeftRotate(x, 17);
	}
	unsigned int P1(unsigned int x) {
		return x ^ LeftRotate(x, 15) ^ LeftRotate(x, 23);
	}
	unsigned int FF(unsigned int j, unsigned int x, unsigned int y, unsigned int z) {
		if (j < 16) return x ^ y ^ z;
		else return (x&y) | (x&z) | (y&z);
	}
	unsigned int GG(unsigned int j, unsigned int x, unsigned int y, unsigned int z) {
		if (j < 16) return x ^ y ^ z;
		else return (x&y) | ((~x)&z) ;
	}
	bigNum SM3(bigNum src) {
		const unsigned int T[64] = {
			0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519,
			0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519, 0x79cc4519,
			0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a,
			0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a,
			0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a,
			0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a,
			0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a,
			0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, 0x7a879d8a, };
		unsigned int h[8] = {
			0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e };
		std::string tmpstr = bigNum(src).hex();
		tmpstr += "8";
		while (tmpstr.length() % 128 != 112) {
			tmpstr += "0";
		}
		tmpstr += hex((unsigned long long)src.hexLen << 2);
		bigNum tmpnum = bigNum(tmpstr);

		for (int i = 0; i < tmpnum.siz; i += 8) {
			unsigned int w[68] = {}, ww[64] = {};
			for (int j = 0; j < 16; j += 2) {
				w[j] = (unsigned int)(tmpnum.num[i + (j >> 1)] >> 32);
				w[j + 1] = (unsigned int)(tmpnum.num[i + (j >> 1)]);
			}
			for (int j = 16; j < 68; j++) {
				w[j] = P1(w[j - 16] ^ w[j - 9] ^ LeftRotate(w[j - 3], 15)) ^ LeftRotate(w[j - 13], 7) ^ w[j - 6];
			}
			for (int j = 0; j < 64; j++) {
				ww[j] = w[j] ^ w[j + 4];
			}
			unsigned int t[8] = {};
			for (int j = 0; j < 8; j++) {
				t[j] = h[j];
			}
			for (int j = 0; j < 64; j++) {
				unsigned int SS1, SS2, TT1, TT2;
				SS1 = LeftRotate(LeftRotate(t[0], 12) + t[4] + LeftRotate(T[j], j), 7);
				SS2 = SS1 ^ LeftRotate(t[0], 12);
				TT1 = FF(j, t[0], t[1], t[2]) + t[3] + SS2 + ww[j];
				TT2 = GG(j, t[4], t[5], t[6]) + t[7] + SS1 + w[j];
				t[7] = t[6];
				t[6] = LeftRotate(t[5], 19);
				t[5] = t[4];
				t[4] = P0(TT2);
				t[3] = t[2];
				t[2] = LeftRotate(t[1],9);
				t[1] = t[0];
				t[0] = TT1;
			}
			for (int j = 0; j < 8; j++) {
				h[j] = h[j] ^ t[j];
			}
		}

		bigNum result = bigNum(4);
		result.num[0] = ((unsigned long long)h[0] << 32) | h[1];
		result.num[1] = ((unsigned long long)h[2] << 32) | h[3];
		result.num[2] = ((unsigned long long)h[4] << 32) | h[5];
		result.num[3] = ((unsigned long long)h[6] << 32) | h[7];
		return result;
	}

	struct smallNum {
		int siz;
		std::vector<unsigned long long>num;
		smallNum(int _siz = 1) {
			siz = _siz;
			num.resize(_siz, 0);
		}
		smallNum(char ch) {
			siz = 1;
			num.push_back(ch - '0');
		}
		smallNum(std::string src) {
			int len = src.length();
			siz = (len + 15) >> 4;
			num.resize(siz, 0);
			for (int i = len - 1, cnt = 0; i >= 0; i--, cnt++) {
				num[cnt >> 4] |= (unsigned long long)dec(src[i]) << ((cnt & 0x0F) << 2);
			}
		}
		smallNum fromBin(std::string src) {
			smallNum result = smallNum();
			int len = src.length();
			if (!len) {
				return smallNum();
			}
			result.siz = (len + 63) >> 6;
			result.num.resize(result.siz, 0);
			for (int i = len - 1, cnt = 0; i >= 0; i--, cnt++) {
				result.num[cnt >> 6] |= (unsigned long long)dec(src[i]) << (cnt & 0x3F);
			}
			return result;
		}
		smallNum(bigNum src) {
			std::string srcstr = src.hex();
			*this = smallNum(srcstr);
		}
		smallNum fromBigNum(bigNum src) {
			std::string srcstr = src.hex();
			return smallNum(srcstr);
		}
		bigNum toBigNum(smallNum src, int minHexLength = 0) {
			std::string srcstr = src.hex();
			return bigNum(srcstr);
		}
		smallNum e() {
			smallNum result = smallNum(1);
			result.num[0] = 1;
			return result;
		}
		smallNum FromU32(uint32_t src) {
			smallNum result = smallNum(1);
			result.num[0] = src;
			return result;
		}
		std::string hex() {
			std::string result = "";
			for (int i = siz - 1; i >= 0; i--) {
				for (int j = 60; j >= 0; j -= 4) {
					result += hexChars[num[i] >> j & 0x0F];
				}
			}
			return result;
		}
		std::string bin() {
			std::string result = "";
			for (int i = siz - 1; i >= 0; i--) {
				for (int j = 63; j >= 0; j--) {
					result += hexChars[num[i] >> j & 0x01];
				}
			}
			return result;
		}
		smallNum& operator^=(const smallNum &rhs) {
			smallNum result = *this;
			int lmt = rhs.siz;
			for (int i = 0; i < lmt; i++) {
				result.num[i] ^= rhs.num[i];
			}
			return *this = result;
		}
		friend smallNum operator^(const smallNum& lhs, const smallNum& rhs) {
			if(lhs.siz>rhs.siz) return smallNum(lhs) ^= rhs;
			else return smallNum(rhs) ^= lhs;
		}
		friend bool operator<(const smallNum& lhs, const smallNum& rhs) {
			if (lhs.siz^rhs.siz) {
				return lhs.siz < rhs.siz;
			}
			for (int i = lhs.siz - 1; i >= 0; i--) {
				if (lhs.num[i] ^ rhs.num[i]) {
					return lhs.num[i] < rhs.num[i];
				}
			}
			return 0;
		}
		friend bool operator<=(const smallNum& lhs, const smallNum& rhs) {
			if (lhs.siz^rhs.siz) {
				return lhs.siz < rhs.siz;
			}
			for (int i = lhs.siz - 1; i >= 0; i--) {
				if (lhs.num[i] ^ rhs.num[i]) {
					return lhs.num[i] < rhs.num[i];
				}
			}
			return 1;
		}
		friend bool operator>(const smallNum& lhs, const smallNum& rhs) {
			return smallNum(rhs) < smallNum(lhs);
		}
		friend bool operator>=(const smallNum& lhs, const smallNum& rhs) {
			return smallNum(rhs) <= smallNum(lhs);
		}
		friend bool operator==(const smallNum& lhs, const smallNum& rhs) {
			if (lhs.siz^rhs.siz) {
				return 0;
			}
			for (int i = lhs.siz - 1; i >= 0; i--) {
				if (lhs.num[i] ^ rhs.num[i]) {
					return 0;
				}
			}
			return 1;
		}
		friend bool operator!=(const smallNum& lhs, const smallNum& rhs) {
			if (lhs.siz^rhs.siz) {
				return 1;
			}
			for (int i = lhs.siz - 1; i >= 0; i--) {
				if (lhs.num[i] ^ rhs.num[i]) {
					return 1;
				}
			}
			return 0;
		}
		smallNum& operator+=(const smallNum& rhs) {
			while (siz < rhs.siz) {
				++siz; num.push_back(0ULL);
			}
			unsigned long long carry0 = 0ULL, carry1 = 0ULL;
			for (int i = 0; i < rhs.siz; i++) {
				if (((num[i] >> 1) + (rhs.num[i] >> 1) + (((num[i] & 1ULL) + (rhs.num[i] & 1ULL) + carry0) >> 1)) >> 63) {
					if (i + 1 == siz) {
						++siz; num.push_back(0ULL);
					}
					carry1 = 1ULL;
				}
				num[i] += rhs.num[i] + carry0;
				carry0 = carry1;
				carry1 = 0ULL;
			}
			for (int i = rhs.siz; i < siz; i++) {
				if (((num[i] >> 1) + (((num[i] & 1ULL) + carry0) >> 1)) >> 63) {
					if (i + 1 == siz) {
						++siz; num.push_back(0ULL);
					}
					carry1 = 1ULL;
				}
				num[i] += carry0;
				carry0 = carry1;
				carry1 = 0ULL;
			}
			return *this;
		}
		friend smallNum operator+(const smallNum& lhs, const smallNum&rhs) {
			return smallNum(lhs) += rhs;
		}
		smallNum& operator-=(const smallNum& rhs){
            unsigned long long tmp,carry0=0,carry1=0;
            for(int i=0;i<rhs.siz;i++){
                tmp=num[i];
                num[i]-=rhs.num[i];
                if(num[i]>tmp){
                    carry1=1;
                }
                tmp=num[i];
                num[i]-=carry0;
                if(num[i]>tmp){
                    carry1=1;
                }
                carry0=carry1;
                carry1=0;
            }
            for(int i=rhs.siz;i<siz;i++){
                tmp=num[i];
                num[i]-=carry0;
                if(num[i]>tmp){
                    carry1=1;
                }
                carry0=carry1;
                carry1=0;
            }
            while(siz>1&&!num[siz-1]){
                --siz;num.pop_back();
            }
            return *this;
        }
		friend smallNum operator-(const smallNum& lhs, const smallNum&rhs) {
			return smallNum(lhs) -= rhs;
		}
		void _add(unsigned long long u,unsigned long long v,unsigned long long &x,unsigned long long &y,unsigned long long &z){
            x+=u; if(x<u) {++y; if(!y) ++z; }
            y+=v; if(y<v) ++z;
        }
        smallNum& operator*=(const smallNum& rhs){
            int SIZ=siz+rhs.siz;
            smallNum result=smallNum(SIZ);
            result.num.push_back(0ULL);
            for(int k=0;k<SIZ;++k){
                for(int i=(k>=rhs.siz?k-rhs.siz+1:0),j=k-i;i<siz&&~j;--j,++i){
                    __uint128_t tmp=(__uint128_t)num[i]*(__uint128_t)rhs.num[j];
                    _add((unsigned long long)(tmp&(-1ULL)),(unsigned long long)((tmp>>64)&(-1ULL)),result.num[k],result.num[k+1],result.num[k+2]);
                }
            }
            result.num.pop_back();
            if(result.siz>1&&!result.num[SIZ-1]){
                --result.siz;
                result.num.pop_back();
            }
            return *this=result;
        }

		friend smallNum operator*(const smallNum& lhs, const smallNum&rhs) {
			return smallNum(lhs) *= rhs;
		}
		smallNum pow(unsigned long long rhs) {
			smallNum result = smallNum().e(), u = smallNum(*this);
			while (rhs) {
				if (rhs & 1) {
					result *= u;
				}
				u *= u;
				rhs >>= 1;
			}
			return result;
		}
		smallNum& operator%=(const smallNum& rhs) {
			smallNum result = smallNum();
			std::string lhsBin = smallNum(*this).bin();
			int lhsLen = lhsBin.length();
			for (int i = 0; i < lhsLen; i++) {
				result = result + result;
				if (lhsBin[i] & 1) {
					result.num[0] |= 1;
				}
				if (rhs <= result) {
					result = result - rhs;
				}
			}
			return *this = result;
		}
		friend smallNum operator%(const smallNum& lhs, const smallNum&rhs) {
			return smallNum(lhs) %= rhs;
		}
		smallNum pow_mod(unsigned long long rhs, smallNum _mod) {
			smallNum result = smallNum().e(), u = smallNum(*this);
			while (rhs) {
				if (rhs & 1) {
					result = (result*u) % _mod;
				}
				u = (u*u) % _mod;
				rhs >>= 1;
			}
			return result;
		}
		smallNum pow_mod(smallNum rhs, smallNum _mod) {
			smallNum result = smallNum().e(), u = smallNum(*this);
			std::string rhsstr = rhs.bin();
			int rhsLen = rhsstr.length();
			for (int i = rhsLen - 1; i >= 0; i--) {
				if (rhsstr[i] & 1) {
					result = (result*u) % _mod;
				}
				u = (u*u) % _mod;
			}
			return result;
		}
		smallNum fromDec(std::string src) {
			if (src == "") {
				return smallNum();
			}
			smallNum result = smallNum();
			int siz = src.length();
			for (int i = 0; i < siz; i++) {
				result *= smallNum("a");
				result += smallNum(src[i]);
			}
			return result;
		}
	};
	smallNum big2Small(bigNum src) {
		std::string srcstr = src.hex();
		return smallNum(srcstr);
	}
	bigNum small2Big(smallNum src, int minHexLength = 0) {
		std::string srcstr = src.hex();
		return bigNum(srcstr);
	}
}

namespace interface {
	using namespace calc;
	smallNum rsa_d(smallNum rsa_e, smallNum d, smallNum n) {
		return rsa_e.pow_mod(d, n);
	}
}

char tmpresult[512];
char* transferFromString(string src) {
	int siz = src.length();
	if (siz < 512) {
		for (int i = 0; i <= siz; i++) {
			tmpresult[i] = src[i];
		}
	}
	return tmpresult;
}

string transferToString(char* src) {
	string result;
	for (int i = 0; src[i]; i++) {
		result += src[i];
	}
	return result;
}

unsigned int randSeed = 0;

unsigned int getNextRand(unsigned int preRand) {
	if (!preRand) return -1;
	unsigned x = preRand;
	x ^= x << 13;
	x ^= x >> 17;
	x ^= x << 5;
	return x;
}

unsigned int getNextRand(unsigned int preRand,int rounds) {
	if (!preRand) return -1;
	unsigned x = preRand;
	for (int i = 0; i < rounds; i++) {
		x = getNextRand(x);
	}
	return x;
}

uint32_t Keys[2];

void sealKeys() {
	Keys[0] ^= getNextRand(12345678, 32);
	Keys[1] ^= getNextRand(87654321, 32);
}

void decryptKeys(uint32_t *keybuf) {
	keybuf[0] = Keys[0] ^ getNextRand(12345678, 32);
	keybuf[1] = Keys[1] ^ getNextRand(87654321, 32);
}

void initKeys(uint32_t *src) {
	randSeed = src[0];
	randSeed ^= getNextRand(randSeed, 16);
	Keys[0] = getNextRand(randSeed, 555); // Ku
	Keys[1] = getNextRand(randSeed, 777); // Kw
	sealKeys();
}

void setKeys(uint32_t *K) {
	memcpy(Keys, K, sizeof Keys);
}

void getKeys(uint32_t *K) {
	memcpy(K, Keys, sizeof Keys);
}

using namespace calc;
string DebugStr;
string Dstr, Nstr, CDstr, NPWstr;

void encryptD() {
	uint32_t Dkeys[2] = {};
	decryptKeys(Dkeys);
	CDstr = (smallNum(Dstr) ^ big2Small(SM3(bigNum().FromU32(Dkeys[0]) || bigNum(Nstr)))).hex();
	Dstr = "";
}

void decryptD() {
	uint32_t Dkeys[2] = {};
	decryptKeys(Dkeys);
	Dstr = (smallNum(CDstr) ^ big2Small(SM3(bigNum().FromU32(Dkeys[0]) || bigNum(Nstr)))).hex();
}

void setCD(char* src, size_t len) {
	char source[MAX_LEN];
	if (len < MAX_LEN) {
		memcpy(source, src, len + 1);
	}
	CDstr = transferToString(source);
}

void setD(char* src, size_t len){
	char source[MAX_LEN];
	if (len < MAX_LEN) {
		memcpy(source, src, len + 1);
	}
	Dstr = transferToString(source);
	encryptD();
}

void setN(char* src, size_t len){
	char source[MAX_LEN];
	if (len < MAX_LEN) {
		memcpy(source, src, len + 1);
	}
	Nstr = transferToString(source);
}

void getCD(char *buf, size_t len) {
	char* result = transferFromString(CDstr);
	if (len > strlen(result)) {
		memcpy(buf, result, strlen(result) + 1);
	}
}

string RSAsource;

void setRSAsource(char* src, size_t len) {
	char source[MAX_LEN];
	if (len < MAX_LEN) {
		memcpy(source, src, len + 1);
	}
	RSAsource = transferToString(source);
}

void getRSAresult(char *buf, size_t len) {
	decryptD();
	string hpw = interface::rsa_d(smallNum(RSAsource), smallNum(Dstr), smallNum(Nstr)).hex();
	uint32_t Dkeys[2] = {};
	decryptKeys(Dkeys);
	string npw = (smallNum(hpw) ^ big2Small(SM3(bigNum().FromU32(Dkeys[0]) || bigNum(Dstr)))).hex();
	char* result = transferFromString(npw);
	Dstr = "";
	if (len > strlen(result)) {
		memcpy(buf, result, strlen(result) + 1);
	}
}

uint32_t _uints[3];

void registerSeed(uint32_t *src){
	randSeed = src[0];
	randSeed ^= getNextRand(randSeed, 16);
	_uints[0] = getNextRand(randSeed, 111) % 2001 + 2000; // x
	_uints[1] = getNextRand(randSeed, 222) % 1001 + 2000; // mu
	_uints[2] = getNextRand(randSeed, 333) % 15 + 1; // k
}

void setNPW(char* src, size_t len) {
	char source[MAX_LEN];
	if (len < MAX_LEN) {
		memcpy(source, src, len + 1);
	}
	NPWstr = transferToString(source);
}

string _xStr, _muStr, _kStr,_nwidStr;

void calcEmbeddingResult() {
	decryptD();
	string hwid = interface::rsa_d(smallNum(RSAsource), smallNum(Dstr), smallNum(Nstr)).hex();
	uint32_t Dkeys[2] = {};
	decryptKeys(Dkeys);
	_nwidStr = (smallNum(hwid) ^ big2Small(SM3(bigNum().FromU32(Dkeys[1]) || bigNum(Dstr)))).hex();
	string hpw = (smallNum(NPWstr) ^ big2Small(SM3(bigNum().FromU32(Dkeys[0]) || bigNum(Dstr)))).hex();
	smallNum hashPart = big2Small(SM3(bigNum(hpw) || bigNum(hwid)).hex());
	_xStr = (smallNum().FromU32(_uints[0])*smallNum("001001001001001001001001001001001001001001001001001001001001001") ^ hashPart).hex();
	_muStr = (smallNum().FromU32(_uints[1])*smallNum("001001001001001001001001001001001001001001001001001001001001001") ^ hashPart).hex();
	_kStr = (smallNum().FromU32(_uints[2])*smallNum("1111111111111111111111111111111111111111111111111111111111111111") ^ hashPart).hex();
	Dstr = "";
	NPWstr = "";
}

void getX(char *buf, size_t len) {
	char* result = transferFromString(_xStr);
	if (len > strlen(result)) {
		memcpy(buf, result, strlen(result) + 1);
	}
}

void getMu(char *buf, size_t len) {
	char* result = transferFromString(_muStr);
	if (len > strlen(result)) {
		memcpy(buf, result, strlen(result) + 1);
	}
}

void getK(char *buf, size_t len) {
	char* result = transferFromString(_kStr);
	if (len > strlen(result)) {
		memcpy(buf, result, strlen(result) + 1);
	}
}

void getNWID(char *buf, size_t len) {
	char* result = transferFromString(_nwidStr);
	if (len > strlen(result)) {
		memcpy(buf, result, strlen(result) + 1);
	}
}
