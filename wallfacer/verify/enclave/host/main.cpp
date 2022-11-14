#include<stdio.h>
#include"enclave.h"
#include<string.h>
#include<string>
#include"Enclave1_u.h"

#define MAX_LEN 1024

using namespace std;

const char hexChars[] = "0123456789abcdef";
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
uint32_t transferToU32(char* src,int base=10) {
	uint32_t result = 0;
	for (int i = 0; src[i]; i++) {
		result = result * base + dec(src[i]);
	}
	return result;
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

int main(int argc, char* argv[]) {
	cc_enclave_t *eid;
	cc_enclave_result_t ret;
	char *path = transferFromString(PATH);
	
	ret = cc_enclave_create(path, AUTO_ENCLAVE_TYPE, 0, SECGEAR_DEBUG_FLAG, NULL, 0, &eid);
	if (ret != CC_SUCCESS) {
		printf("ERR: %#x ,failed to create enclave.\n", ret);
		return -1;
	}
	char buffer[MAX_LEN];
	uint32_t uints[2] = {};
	if (argc > 1) {
		if(strcmp(argv[1],"initServer")==0){
			uints[0] = transferToU32(argv[4]);
			initKeys(eid, uints);
			setN(eid, argv[3], strlen(argv[3]) + 1);
			setD(eid, argv[2], strlen(argv[2]) + 1);
			getCD(eid, buffer, MAX_LEN); 
			printf("%s\n", buffer); 
			getKeys(eid, uints); 
			printf("%u\n%u\n", uints[0], uints[1]); 
		}
		else if(strcmp(argv[1], "passwordEncryption") == 0){
			setN(eid, argv[4], strlen(argv[4]) + 1);
			setCD(eid, argv[3], strlen(argv[3]) + 1);
			setRSAsource(eid, argv[2], strlen(argv[2]) + 1);
			uints[0] = transferToU32(argv[5]);
			setKeys(eid, uints);
			getRSAresult(eid, buffer, MAX_LEN);
			printf("%s\n", buffer);
		}
		else if(strcmp(argv[1], "embedding") == 0){
			setN(eid, argv[5], strlen(argv[5]) + 1);
			setCD(eid, argv[4], strlen(argv[4]) + 1);
			setNPW(eid, argv[3], strlen(argv[3]) + 1);
			setRSAsource(eid, argv[2], strlen(argv[2]) + 1);
			uints[0] = transferToU32(argv[8]);
			registerSeed(eid, uints);
			uints[0] = transferToU32(argv[6]);
			uints[1] = transferToU32(argv[7]);
			setKeys(eid, uints);
			calcEmbeddingResult(eid);
			getX(eid, buffer, MAX_LEN);
			printf("%s\n", buffer);
			getMu(eid, buffer, MAX_LEN);
			printf("%s\n", buffer);
			getK(eid, buffer, MAX_LEN);
			printf("%s\n", buffer);
			getNWID(eid, buffer, MAX_LEN);
			printf("%s\n", buffer);
		}
	}

	if (CC_SUCCESS != cc_enclave_destroy(eid)) {
		return -1;
	}
	return 0;
}
